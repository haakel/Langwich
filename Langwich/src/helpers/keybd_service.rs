//! Keyboard simulation service
//!
//! Provides safe wrappers for keyboard input simulation using Windows APIs

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, KEYBDINPUT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    VkKeyScanExW, GetKeyboardLayout, MapVirtualKeyExW, MAPVK_VK_TO_VSC,
};

/// Virtual Key (VK) codes
pub const VK_C: u16 = 0x43;      // Ctrl+C
pub const VK_V: u16 = 0x56;      // Ctrl+V
pub const VK_LCONTROL: u16 = 0xA1;
pub const VK_RCONTROL: u16 = 0xA2;
pub const VK_LSHIFT: u16 = 0xA0;
pub const VK_RSHIFT: u16 = 0xA1;

/// Simulates a keyboard shortcut with specified modifiers
/// # Arguments
/// * `vkey` - Virtual key code of the main key
/// * `modifiers` - Bitmask of modifier flags (1=Alt, 2=Ctrl, 4=Shift, 8=Windows)
pub fn simulate_keyboard_shortcut(vkey: u16, modifiers: u32) -> bool {
    let mut inputs: [INPUT; 3] = unsafe { std::mem::zeroed() };
    
    // Press modifiers first
    let mut mod_index = 0;
    if has_modifier(modifiers, 1) { // Alt
        inputs[mod_index].u().ki_mut().wVk = 0xA4; // VK_MENU
        inputs[mod_index].u().ki_mut().dwFlags = 0; // KEYEVENTF_KEYDOWN
        mod_index += 1;
    }
    
    if has_modifier(modifiers, 2) { // Control
        inputs[mod_index].u().ki_mut().wVk = if modifiers & 3 != 0 {
            VK_RCONTROL
        } else {
            VK_LCONTROL
        };
        inputs[mod_index].u().ki_mut().dw_flags = 0;
        mod_index += 1;
    }
    
    if has_modifier(modifiers, 4) { // Shift
        inputs[mod_index].u().ki_mut().wVk = if modifiers & 5 != 0 {
            VK_RSHIFT
        } else {
            VK_LSHIFT
        };
        inputs[mod_index].u().ki_mut().dw_flags = 0;
        mod_index += 1;
    }
    
    // Press the main key
    inputs[mod_index].u().ki_mut().wVk = vkey;
    inputs[mod_index].u().ki_mut().dw_flags = 0;
    
    // Release keys in reverse order
    let release_start = mod_index + 1;
    let mut release_index = release_start;
    
    // Release main key
    inputs[release_index].u().ki_mut().wVk = vkey;
    inputs[release_index].u().ki_mut().dw_flags = 2; // KEYEVENTF_KEYUP
    release_index += 1;
    
    // Release modifiers in reverse
    if has_modifier(modifiers, 4) { // Shift
        inputs[release_index].u().ki_mut().wVk = if modifiers & 5 != 0 {
            VK_RSHIFT
        } else {
            VK_LSHIFT
        };
        inputs[release_index].u().ki_mut().dw_flags = 2;
        release_index += 1;
    }
    
    if has_modifier(modifiers, 2) { // Control
        inputs[release_index].u().ki_mut().wVk = if modifiers & 3 != 0 {
            VK_RCONTROL
        } else {
            VK_LCONTROL
        };
        inputs[release_index].u().ki_mut().dw_flags = 2;
        release_index += 1;
    }
    
    if has_modifier(modifiers, 1) { // Alt
        inputs[release_index].u().ki_mut().wVk = 0xA4; // VK_MENU
        inputs[release_index].u().ki_mut().dw_flags = 2;
        release_index += 1;
    }
    
    // Calculate total number of inputs
    let total_inputs = release_index as u32;
    
    // Send the input events
    let result = unsafe { SendInput(&inputs[..total_inputs as usize]) };
    
    result == total_inputs
}

/// Simulates just pressing a single key (no modifiers)
pub fn press_key(vkey: u16) -> bool {
    let mut input: INPUT = unsafe { std::mem::zeroed() };
    
    unsafe {
        *input.u_mut() = INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vkey,
                wScan: 0,
                dwFlags: 0,
                time: 0,
                dwExtraInfo: 0,
            },
        };
    }
    
    let result = unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
    result > 0
}

/// Simulates releasing a key
pub fn release_key(vkey: u16) -> bool {
    let mut input: INPUT = unsafe { std::mem::zeroed() };
    
    unsafe {
        *input.u_mut() = INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vkey,
                wScan: 0,
                dwFlags: 2, // KEYEVENTF_KEYUP
                time: 0,
                dwExtraInfo: 0,
            },
        };
    }
    
    let result = unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
    result > 0
}

/// Helper function to check if a modifier bit is set
fn has_modifier(mask: u32, bit: u32) -> bool {
    (mask & bit) != 0
}

/// Gets current thread's keyboard layout handle
pub fn get_keyboard_layout() -> Option<u32> {
    unsafe {
        let hkl = GetKeyboardLayout(0);
        if hkl.0 != 0 {
            Some(hkl.0 as u32)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_modifier() {
        assert!(has_modifier(1, 1));
        assert!(!has_modifier(0, 1));
        
        assert!(has_modifier(3, 1));
        assert!(has_modifier(3, 2));
        assert!(!has_modifier(3, 4));
    }

    #[test]
    fn test_vkey_codes() {
        assert_eq!(VK_C, 0x43);
        assert_eq!(VK_V, 0x56);
    }
}
