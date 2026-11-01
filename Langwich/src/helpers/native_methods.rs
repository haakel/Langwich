//! Low-level Win32 API wrapper functions
//!
//! Provides safe Rust wrappers for essential Win32 functions used in Langwich

use std::ffi::{c_void, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Registry::{RegCreateKeyExW, RegDeleteValueW, RegSetValueExW, HKEY, HCERTSTORE, REG_BINARY, REG_DWORD, REG_SZ};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, IsIconic, RegisterHotKey, UnregisterHotKey,
    MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN, WM_HOTKEY,
};

pub const VK_F10: u32 = 0x79;
pub const VK_RCONTROL: u32 = 0xA2;
pub const VK_LCONTROL: u32 = 0xA1;
pub const WM_USER: u32 = 0x0400;

/// Windows message constants
pub const WM_QUERYENDSESSION: u32 = 0x0011;
pub const WM_ENDSESSION: u32 = 0x0016;

/// Clipboard sequence number (stored in registry)
pub const CLIPBOARD_SEQ_KEY_NAME: &str = "LangwichClipboardSequence";

#[derive(Debug)]
pub enum RegistryError {
    NotSupportedOnPlatform,
    AccessDenied,
    NotFound,
    Other(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::NotSupportedOnPlatform => write!(f, "Registry operations not supported on this platform"),
            RegistryError::AccessDenied => write!(f, "Access denied"),
            RegistryError::NotFound => write!(f, "Key or value not found"),
            RegistryError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for RegistryError {}

/// Converts string to wide string (UTF-16)
pub fn string_to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0))
        .collect()
}

/// Wide string to Rust String
pub fn wide_to_string(wide: &[u16]) -> String {
    OsStr::decode_wide(wide)
        .take_while(|&c| c != 0)
        .collect::<String>()
}

// ============================================================================
// Window Management Functions
// ============================================================================

/// Gets handle to foreground window
pub fn get_foreground_window() -> Option<HWND> {
    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.0 == 0 {
        None
    } else {
        Some(hwnd)
    }
}

/// Checks if window is minimized
pub fn is_window_minimized(hwnd: HWND) -> bool {
    unsafe { IsIconic(hwnd).as_bool() }
}

/// Gets process ID from window handle
pub fn get_window_process_id(hwnd: HWND) -> Option<u32> {
    let mut pid = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }
    if pid == 0 {
        None
    } else {
        Some(pid)
    }
}

// ============================================================================
// Hotkey Functions
// ============================================================================

/// Registers a global hotkey
/// # Arguments
/// * `hwnd` - Handle to window that will receive WM_HOTKEY messages
/// * `id` - Hotkey identifier (unique within application)
/// * `modifiers` - Modifier flags (MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN)
/// * `vkey` - Virtual key code
pub fn register_hotkey(hwnd: isize, id: u32, modifiers: u32, vkey: u32) -> Result<(), RegistryError> {
    let result = unsafe { RegisterHotKey(HWND(hwnd as usize), id, modifiers, vkey) };
    
    if result.is_ok() {
        Ok(())
    } else {
        Err(RegistryError::Other(format!("Failed to register hotkey: {}", result.unwrap_err())))
    }
}

/// Unregisters a global hotkey
pub fn unregister_hotkey(hwnd: isize, id: u32) -> Result<(), RegistryError> {
    let result = unsafe { UnregisterHotKey(HWND(hwnd as usize), id) };
    
    if result.is_ok() {
        Ok(())
    } else {
        Err(RegistryError::Other(format!("Failed to unregister hotkey: {}", result.unwrap_err())))
    }
}

// ============================================================================
// Modifier Key Utilities
// ============================================================================

/// Combines modifier flags
#[inline]
pub fn combine_modifiers(base: u32, extra: u32) -> u32 {
    base | extra
}

/// Check if Alt modifier is set
#[inline]
pub fn has_alt(modifiers: u32) -> bool {
    (modifiers & MOD_ALT.0 as u32) != 0
}

/// Check if Control modifier is set
#[inline]
pub fn has_control(modifiers: u32) -> bool {
    (modifiers & MOD_CONTROL.0 as u32) != 0
}

/// Check if Shift modifier is set
#[inline]
pub fn has_shift(modifiers: u32) -> bool {
    (modifiers & MOD_SHIFT.0 as u32) != 0
}

/// Check if Windows key modifier is set
#[inline]
pub fn has_windows(modifiers: u32) -> bool {
    (modifiers & MOD_WIN.0 as u32) != 0
}

/// Convert WPF ModifierKeys to Windows modifier flags
pub fn wpf_modifiers_to_windows(modifier_mask: u32) -> u32 {
    let mut flags = 0u32;
    
    if (modifier_mask & 1) != 0 { // Alt
        flags |= MOD_ALT.0 as u32;
    }
    if (modifier_mask & 2) != 0 { // Control
        flags |= MOD_CONTROL.0 as u32;
    }
    if (modifier_mask & 4) != 0 { // Shift
        flags |= MOD_SHIFT.0 as u32;
    }
    if (modifier_mask & 8) != 0 { // Windows
        flags |= MOD_WIN.0 as u32;
    }
    
    flags
}

// ============================================================================
// Clipboard Sequence Number
// ============================================================================

/// Gets the current clipboard sequence number from registry
pub fn get_clipboard_sequence_number() -> Result<u32, RegistryError> {
    let mut hkey = HKEY(0);
    let path = format!("Software\\{}\\{}", "Langwich", CLIPBOARD_SEQ_KEY_NAME);
    let path_wide = string_to_wide(&path);
    
    let status = unsafe {
        RegCreateKeyExW(
            windows::Win32::Registry::HKEY_CURRENT_USER,
            PCWSTR(path_wide.as_ptr()),
            0,
            None,
            0,
            windows::Win32::Registry::KEY_READ,
            None,
            &mut hkey,
            None,
        )
    };
    
    if status.is_ok() {
        let mut data_type = 0;
        let mut data_size = 4;
        let mut data: [u8; 4] = [0; 4];
        
        let status = unsafe {
            windows::Win32::Registry::RegQueryValueExW(
                hkey,
                PCWSTR(path_wide.as_ptr()),
                None,
                Some(&mut data_type),
                Some(data.as_mut_ptr().cast()),
                Some(&mut data_size),
            )
        };
        
        if status.is_ok() {
            return Ok(u32::from_le_bytes(data));
        }
    }
    
    Ok(0)
}

/// Sets the clipboard sequence number in registry
pub fn set_clipboard_sequence_number(seq: u32) -> Result<(), RegistryError> {
    let mut hkey = HKEY(0);
    let path = format!("Software\\{}\\{}", "Langwich", CLIPBOARD_SEQ_KEY_NAME);
    let path_wide = string_to_wide(&path);
    
    let status = unsafe {
        RegCreateKeyExW(
            windows::Win32::Registry::HKEY_CURRENT_USER,
            PCWSTR(path_wide.as_ptr()),
            0,
            None,
            0,
            windows::Win32::Registry::KEY_WRITE,
            None,
            &mut hkey,
            None,
        )
    };
    
    if status.is_ok() {
        let data_type = REG_DWORD;
        let data = seq.to_le_bytes();
        
        let status = unsafe {
            RegSetValueExW(
                hkey,
                PCWSTR(path_wide.as_ptr()),
                0,
                data_type,
                Some(data.as_ptr()),
                data.len() as u32,
            )
        };
        
        if status.is_ok() {
            return Ok(());
        }
    }
    
    Err(RegistryError::Other("Failed to set clipboard sequence".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_combinations() {
        assert!(!has_alt(0));
        assert!(has_alt(MOD_ALT.0 as u32));
        assert!(has_alt(MOD_ALT.0 as u32 | MOD_CONTROL.0 as u32));
        
        assert!(!has_control(0));
        assert!(has_control(MOD_CONTROL.0 as u32));
        
        assert!(!has_shift(0));
        assert!(has_shift(MOD_SHIFT.0 as u32));
        
        assert!(!has_windows(0));
        assert!(has_windows(MOD_WIN.0 as u32));
    }

    #[test]
    fn test_string_conversion() {
        let test_str = "Test123!@#";
        let wide = string_to_wide(test_str);
        let back = wide_to_string(&wide);
        assert_eq!(back, test_str);
    }
}
