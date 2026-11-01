//! Hidden host window for message processing
//!
//! Provides an invisible window to receive system callbacks (hotkey messages, etc.)

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, BOOL, POINT};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, RegisterClassExW, UnregisterClassW, DefWindowProcW,
    MSG, WNDCLASSESW, CS_HREDRAW, CS_VREDRAW,
    CW_USEDEFAULT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, 
    GetMessageW, TranslateMessage, DispatchMessageW, PostQuitMessage,
    HINSTANCE, HCURSOR, HBRUSH, HICON, LR_DEFAULTSIZE,
    IDC_ARROW, SW_HIDE, IDI_APPLICATION, LoadCursorW, LoadIconW,
    WM_DESTROY, WM_CLOSE, WM_SYSKEYDOWN, WM_KEYDOWN,
};
use crate::helpers::native_methods::string_to_wide;

/// Message types processed by the hidden host window
#[derive(Debug, Clone)]
pub enum WindowMessage {
    /// Hotkey was pressed
    HotkeyPressed,
    
    /// Settings request
    ShowSettings,
    
    /// Exit request
    Exit,
}

/// Hidden host window implementation
pub struct HiddenHostWindow {
    /// Window handle
    hwnd: Option<HWND>,
    
    /// Current window class atom (for registration)
    class_atom: u16,
}

impl Default for HiddenHostWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl HiddenHostWindow {
    /// Creates a new hidden host window
    pub fn new() -> Self {
        Self {
            hwnd: None,
            class_atom: 0,
        }
    }

    /// Registers the window class if not already registered
    fn ensure_class_registered(&mut self) -> Result<u16, String> {
        if self.class_atom != 0 {
            return Ok(self.class_atom);
        }

        // Get application instance
        let hinstance = unsafe { HINSTANCE(GetModuleHandleW(None).ok()?.0 as usize) };

        let class_name = string_to_wide("LangwichHiddenWindow");
        
        let wc = WNDCLASSESW {
            cbSize: std::mem::size_of::<WNDCLASSESW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc_callback),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: unsafe { LoadIconW(None, IDI_APPLICATION).ok()? },
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }.ok()?,
            hbrBackground: unsafe { HBRUSH(0) },
            lpszMenuName: None,
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hIconSm: None,
        };

        let atom = unsafe { RegisterClassExW(&wc) };
        
        if atom == 0 {
            return Err("Failed to register window class".to_string());
        }

        self.class_atom = atom;
        Ok(atom)
    }

    /// Creates the hidden window
    pub fn create(&mut self) -> Result<(), String> {
        if self.hwnd.is_some() {
            return Ok(()); // Already created
        }

        self.ensure_class_registered()?;

        let class_name = string_to_wide("LangwichHiddenWindow");
        
        let hwnd = unsafe {
            CreateWindowExW(
                0,
                PCWSTR(class_name.as_ptr()),
                None,
                WS_OVERLAPPEDWINDOW & !WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                None,
                None,
                HINSTANCE(0), // Will be set properly in callback
                None,
            )?
        };

        self.hwnd = Some(hwnd);
        Ok(())
    }

    /// Gets the window handle
    pub fn hwnd(&self) -> Option<usize> {
        self.hwnd.map(|h| h.0)
    }

    /// Processes Windows messages
    pub fn process_messages(&mut self, timeout_ms: i32) -> Result<Option<WindowMessage>, String> {
        if let Some(hwnd) = self.hwnd {
            unsafe {
                let mut msg: MSG = std::mem::zeroed();
                
                let result = GetMessageW(&mut msg, hwnd, 0, 0);
                
                if result.as_bool() {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                    
                    // Check for specific message types
                    if msg.message == WM_DESTROY || msg.message == WM_CLOSE {
                        return Ok(Some(WindowMessage::Exit));
                    } else if msg.wParam.0 == 9001 { // Hotkey ID
                        return Ok(Some(WindowMessage::HotkeyPressed));
                    }
                }
            }
        }
        
        Ok(None)
    }

    /// Posts a message to itself (for signaling)
    pub fn post_message(&self, message_id: u32, wparam: u32) -> Result<(), String> {
        if let Some(hwnd) = self.hwnd {
            unsafe {
                use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
                PostMessageW(HWND(hwnd), message_id, WPARAM(wparam), LPARAM(0)).ok()?;
            }
            Ok(())
        } else {
            Err("Window not created".to_string())
        }
    }

    /// Runs the message loop
    pub fn run_message_loop<F>(&mut self, handler: F) -> Result<(), String>
    where
        F: Fn(&WindowMessage),
    {
        unsafe {
            while self.process_messages(-1)?.is_some() {
                // Message loop will be driven externally
            }
        }
        Ok(())
    }

    /// Stops the message loop
    pub fn stop(&self) {
        if let Some(hwnd) = self.hwnd {
            unsafe {
                PostQuitMessage(0);
            }
        }
    }
}

// Callback function for window procedure
extern "system" fn wnd_proc_callback(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
    }
}

/// Global callback for all instances
static GLOBAL_CALLBACK: unsafe fn(HWND, u32, WPARAM, LPARAM) -> LRESULT = wnd_proc_callback;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let mut window = HiddenHostWindow::new();
        assert!(window.create().is_ok());
        assert!(window.hwnd().is_some());
    }

    #[test]
    fn test_message_processing() {
        let mut window = HiddenHostWindow::new();
        window.create().unwrap();
        
        // Should not panic
        let result = window.process_messages(0);
        assert!(result.is_ok());
    }
}
