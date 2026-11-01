//! Global hotkey service for Windows
//!
//! Manages global hotkey registration using Windows API

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use parking_lot::Mutex;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL, MOD_SHIFT, MOD_WIN, WM_HOTKEY,
    GetModuleHandleW, LoadLibraryW, GetProcAddress, PostMessageW, HINSTANCE,
};

/// Event types sent from hotkey handler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotkeyEvent {
    /// Hotkey was pressed
    Pressed,
    
    /// Error occurred
    Error(String),
}

/// Service for managing global hotkeys
pub struct HotkeyService {
    /// Window handle for receiving messages
    hwnd: isize,
    
    /// Hotkey identifier (unique within application)
    hotkey_id: u32,
    
    /// Whether hotkey is currently registered
    is_registered: Arc<AtomicBool>,
    
    /// Current modifiers
    current_modifiers: Mutex<Option<u32>>,
}

impl Default for HotkeyService {
    fn default() -> Self {
        Self::new(9001)
    }
}

impl HotkeyService {
    /// Creates a new instance with specified ID
    pub fn new(hotkey_id: u32) -> Self {
        // Create a hidden window for receiving messages
        let hwnd = Self::create_hidden_window();
        
        Self {
            hwnd,
            hotkey_id,
            is_registered: Arc::new(AtomicBool::new(false)),
            current_modifiers: Mutex::new(None),
        }
    }

    /// Creates a hidden window for message processing
    fn create_hidden_window() -> isize {
        // In a real implementation, we would create a proper HWND
        // For now, we'll use 0 and add proper window creation later
        // This is a placeholder
        0 as isize
    }

    /// Registers a global hotkey
    /// # Arguments
    /// * `modifiers` - Bitmask of modifier flags (1=Alt, 2=Ctrl, 4=Shift, 8=Windows key)
    /// * `vkey` - Virtual key code
    pub fn register_hotkey(&self, modifiers: u32, vkey: u16) -> Result<(), String> {
        self.unregister_hotkey_internal()?;
        
        let is_success = unsafe {
            RegisterHotKey(HWND(self.hwnd as usize), self.hotkey_id, modifiers, vkey)
        };
        
        if is_success.as_bool() {
            self.is_registered.store(true, Ordering::SeqCst);
            *self.current_modifiers.lock() = Some(modifiers);
            Ok(())
        } else {
            let error = format!("Failed to register hotkey: mod=0x{:X}, vk=0x{:X}", modifiers, vkey);
            Err(error)
        }
    }

    /// Unregisters the current hotkey
    pub fn unregister_hotkey(&mut self) {
        let _ = self.unregister_hotkey_internal();
    }

    /// Internal unregister function
    fn unregister_hotkey_internal(&self) -> Result<(), String> {
        if self.is_registered.load(Ordering::SeqCst) {
            let result = unsafe {
                UnregisterHotKey(HWND(self.hwnd as usize), self.hotkey_id)
            };
            
            if result.as_bool() {
                self.is_registered.store(false, Ordering::SeqCst);
                *self.current_modifiers.lock() = None;
                Ok(())
            } else {
                Err(format!("Failed to unregister hotkey"))
            }
        } else {
            Ok(())
        }
    }

    /// Gets the current hotkey ID
    pub fn hotkey_id(&self) -> u32 {
        self.hotkey_id
    }

    /// Checks if hotkey is currently registered
    pub fn is_registered(&self) -> bool {
        self.is_registered.load(Ordering::SeqCst)
    }

    /// Gets the current window handle
    pub fn hwnd(&self) -> isize {
        self.hwnd
    }

    /// Handles WM_HOTKEY message
    pub fn handle_hotkey_message(&self, wparam: isize) -> bool {
        if wparam as u32 == self.hotkey_id && self.is_registered.load(Ordering::SeqCst) {
            return true;
        }
        false
    }
}

impl Drop for HotkeyService {
    fn drop(&mut self) {
        self.unregister_hotkey();
    }
}

/// Modifier key representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub alt: bool,
    pub control: bool,
    pub shift: bool,
    pub windows: bool,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            alt: false,
            control: false,
            shift: false,
            windows: false,
        }
    }
}

impl Modifiers {
    /// Combines modifiers into Windows flag value
    pub fn to_windows_flags(&self) -> u32 {
        let mut flags = 0u32;
        
        if self.alt {
            flags |= MOD_ALT.0 as u32;
        }
        if self.control {
            flags |= MOD_CONTROL.0 as u32;
        }
        if self.shift {
            flags |= MOD_SHIFT.0 as u32;
        }
        if self.windows {
            flags |= MOD_WIN.0 as u32;
        }
        
        flags
    }

    /// Parses flags back into Modifiers
    pub fn from_windows_flags(flags: u32) -> Self {
        Self {
            alt: (flags & MOD_ALT.0 as u32) != 0,
            control: (flags & MOD_CONTROL.0 as u32) != 0,
            shift: (flags & MOD_SHIFT.0 as u32) != 0,
            windows: (flags & MOD_WIN.0 as u32) != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = HotkeyService::new(9001);
        assert!(!service.is_registered());
    }

    #[test]
    fn test_modifier_conversions() {
        let mods = Modifiers {
            alt: true,
            control: false,
            shift: true,
            windows: false,
        };
        
        let flags = mods.to_windows_flags();
        let parsed = Modifiers::from_windows_flags(flags);
        
        assert_eq!(mods.alt, parsed.alt);
        assert_eq!(mods.shift, parsed.shift);
    }

    #[test]
    fn test_modifier_combinations() {
        // Test all combinations
        assert_eq!(Modifiers::default().to_windows_flags(), 0);
        
        let mods = Modifiers::from_windows_flags(MOD_ALT.0 as u32);
        assert!(mods.alt);
        
        let mods = Modifiers::from_windows_flags(MOD_CONTROL.0 as u32);
        assert!(mods.control);
        
        let mods = Modifiers::from_windows_flags(MOD_SHIFT.0 as u32);
        assert!(mods.shift);
        
        let mods = Modifiers::from_windows_flags(MOD_WIN.0 as u32);
        assert!(mods.windows);
    }
}
