//! System tray icon service
//!
//! Provides system tray functionality with context menu

use std::sync::{Arc, mpsc};
use parking_lot::Mutex;
use windows::Win32::UI::WindowsAndMessaging::{
    Shell_NotifyIconW, NIF_MESSAGE, NIF_ICON, NIF_TIP, NIF_INFO,
    NIM_ADD, NIM_DELETE, NIM_MODIFY,
    NOTIFYICONDATAW, SW_HIDE,
    IDI_APPLICATION, LoadIconW,
};
use crate::helpers::native_methods::{string_to_wide, wide_to_string};

/// Events sent from tray icon service
#[derive(Debug, Clone)]
pub enum TrayEvent {
    /// Settings button was clicked
    ShowSettings,
    
    /// Exit button was clicked
    ExitRequested,
}

/// Service for managing system tray icon
pub struct TrayIconService {
    /// Icon identifier (unique within application)
    icon_id: u32,
    
    /// Window handle for callback messages
    hwnd: isize,
    
    /// Sender for tray events
    event_sender: Mutex<Option<mpsc::Sender<TrayEvent>>>,
    
    /// Is tray icon currently visible
    is_visible: Arc<Mutex<bool>>,
}

impl Default for TrayIconService {
    fn default() -> Self {
        Self::new(1000, 0 as isize)
    }
}

impl TrayIconService {
    /// Creates a new instance
    pub fn new(icon_id: u32, hwnd: isize) -> Self {
        Self {
            icon_id,
            hwnd,
            event_sender: Mutex::new(None),
            is_visible: Arc::new(Mutex::new(false)),
        }
    }

    /// Sets up event communication channel
    pub fn set_event_sender(&mut self, sender: mpsc::Sender<TrayEvent>) {
        *self.event_sender.lock() = Some(sender);
    }

    /// Adds/initializes the tray icon
    pub fn add_tray_icon(&self, app_name: &str, tooltip: &str) -> Result<(), String> {
        if !self.hwnd.is_null() && !self.hwnd.is_zero() {
            let mut ndata = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.hwnd as *mut _,
                uID: self.icon_id,
                uFlags: NIF_MESSAGE.0 | NIF_ICON.0 | NIF_TIP.0,
                uCallbackMessage: 0x400 + self.icon_id, // WM_USER + ID
                hIcon: unsafe { LoadIconW(None, IDI_APPLICATION).ok()? },
                szTip: [0; 128],
                dwStateMask: 0,
                uTimeout: 0,
                hBalloonIcon: None,
                szInfo: [0; 256],
                dwInfoFlags: 0,
                GuidItem: Default::default(),
                cbSizeTitle: 0,
                szTitle: [0; 64],
                reserved2: [0; 2],
                dwReserved2: 0,
            };
            
            let title = string_to_wide(app_name);
            let tip = string_to_wide(tooltip);
            
            unsafe {
                let data_len = std::cmp::min(title.len().saturating_sub(1), 63);
                ndata.cbSizeTitle = data_len as u32;
                for (i, ch) in title[..data_len].iter().enumerate() {
                    ndata.szTitle[i] = *ch;
                }
                
                let tip_len = std::cmp::min(tip.len().saturating_sub(1), 127);
                for (i, ch) in tip[..tip_len].iter().enumerate() {
                    ndata.szTip[i] = *ch;
                }
                
                Shell_NotifyIconW(NIM_ADD, &ndata).ok()?;
            }
            
            *self.is_visible.lock() = true;
            Ok(())
        } else {
            Err("Invalid window handle".to_string())
        }
    }

    /// Removes the tray icon
    pub fn remove_tray_icon(&self) -> Result<(), String> {
        if !self.hwnd.is_null() && !self.hwnd.is_zero() {
            unsafe {
                Shell_NotifyIconW(NIM_DELETE, &NOTIFYICONDATAW {
                    cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                    hWnd: self.hwnd as *mut _,
                    uID: self.icon_id,
                    ..Default::default()
                }).ok()?;
            }
            *self.is_visible.lock() = false;
            Ok(())
        } else {
            Ok(())
        }
    }

    /// Shows a balloon tip notification
    pub fn show_balloon_tip(&self, title: &str, message: &str) -> Result<(), String> {
        if !self.hwnd.is_null() && !self.hwnd.is_zero() {
            let mut ndata = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.hwnd as *mut _,
                uID: self.icon_id,
                uFlags: NIF_INFO.0,
                ..Default::default()
            };
            
            let title_wide = string_to_wide(title);
            let message_wide = string_to_wide(message);
            
            unsafe {
                let title_len = std::cmp::min(title_wide.len().saturating_sub(1), 63);
                let msg_len = std::cmp::min(message_wide.len().saturating_sub(1), 255);
                
                for (i, ch) in title_wide[..title_len].iter().enumerate() {
                    ndata.szTitle[i] = *ch;
                }
                
                for (i, ch) in message_wide[..msg_len].iter().enumerate() {
                    ndata.szInfo[i] = *ch;
                }
                
                Shell_NotifyIconW(NIM_MODIFY, &ndata).ok()?;
            }
            
            Ok(())
        } else {
            Err("Invalid window handle".to_string())
        }
    }

    /// Notifies about a conversion success
    pub fn show_conversion_success(&self, original: &str, converted: &str) {
        let message = format!(
            "Converted:\n{}\n\n→\n{}",
            original.trim(),
            converted.trim()
        );
        
        let _ = self.show_balloon_tip("Langwich", &message);
    }

    /// Gets current visibility state
    pub fn is_visible(&self) -> bool {
        *self.is_visible.lock()
    }

    /// Sends event to receiver
    pub fn send_event(&self, event: TrayEvent) {
        if let Some(sender) = self.event_sender.lock().take() {
            let _ = sender.send(event);
        }
    }
}

impl Drop for TrayIconService {
    fn drop(&mut self) {
        let _ = self.remove_tray_icon();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = TrayIconService::new(1000, 0);
        assert!(!service.is_visible());
    }
}
