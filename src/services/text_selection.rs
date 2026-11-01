//! Text selection and clipboard manipulation service
//!
//! Handles reading selected text and replacing it with converted content

use std::sync::Arc;
use std::time::Duration;
use parking_lot::Mutex;
use tokio::sync::Semaphore;
use windows::Win32::UI::WindowsAndMessaging::{
    CloseClipboard, OpenClipboard, EmptyClipboard, SetClipboardData, GetClipboardData,
    CF_TEXT, GlobalLock, GlobalUnlock, GHND, IsClipboardFormatAvailable,
};

/// Service for managing text selection and clipboard operations
pub struct TextSelectionService {
    /// Lock to prevent concurrent operations
    lock: Arc<Semaphore>,
    
    /// Clipboard sequence number (for detecting changes)
    clipboard_sequence: Mutex<u32>,
}

impl Default for TextSelectionService {
    fn default() -> Self {
        Self::new()
    }
}

impl TextSelectionService {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            lock: Arc::new(Semaphore::const_new(1)),
            clipboard_sequence: Mutex::new(0),
        }
    }

    /// Gets currently selected text from any window
    /// This is a simplified version that uses clipboard polling
    pub async fn get_selected_text_async(&self) -> Option<String> {
        if !self.lock.acquire().try_owned().is_ok() {
            return None;
        }
        
        // For now, we use a simple approach - check clipboard periodically
        // A full implementation would need Windows hooks or similar
        self.poll_clipboard_for_selection().await
    }

    /// Replaces selected text with new content
    pub async fn replace_selected_text_async(&self, new_text: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.lock.acquire().try_owned().is_ok() {
            return Ok(());
        }
        
        // Set clipboard to new text
        self.set_clipboard_text(new_text)?;
        
        // Small delay to allow system to process
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // In a full implementation, we would simulate Ctrl+V here
        // using keybd_service
        
        Ok(())
    }

    /// Polls clipboard for selected text with timeout
    async fn poll_clipboard_for_selection(&self) -> Option<String> {
        const TIMEOUT_MS: u64 = 500;
        const INTERVAL_MS: u64 = 30;
        
        let mut elapsed = 0u64;
        while elapsed < TIMEOUT_MS {
            tokio::time::sleep(Duration::from_millis(INTERVAL_MS)).await;
            elapsed += INTERVAL_MS;
            
            match self.get_clipboard_text() {
                Some(text) if !text.is_empty() => return Some(text),
                _ => continue,
            }
        }
        
        None
    }

    /// Reads text from clipboard safely
    fn get_clipboard_text(&self) -> Option<String> {
        unsafe {
            if OpenClipboard(None).is_err() {
                return None;
            }
            
            let result = self.read_from_clibpbrd_internal();
            CloseClipboard().ok();
            
            result
        }
    }

    /// Internal clipboard reader (called when clipboard is open)
    unsafe fn read_from_clibpbrd_internal(&self) -> Option<String> {
        if IsClipboardFormatAvailable(CF_TEXT).as_bool() {
            let handle = GetClipboardData(CF_TEXT)?;
            if handle.0 == 0 {
                return None;
            }
            
            let data = GlobalLock(handle)?;
            if data.is_null() {
                return None;
            }
            
            // Convert C string to Rust String
            let c_str = std::ffi::CStr::from_ptr(data as *const i8);
            let result = c_str.to_string_lossy().to_string();
            
            GlobalUnlock(handle).ok();
            
            Some(result)
        } else {
            None
        }
    }

    /// Writes text to clipboard
    fn set_clipboard_text(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            if OpenClipboard(None).is_err() {
                return Err("Failed to open clipboard".into());
            }
            
            EmptyClipboard()?;
            
            let bytes = text.as_bytes();
            let len = bytes.len() + 1;
            
            let handle = GlobalAlloc(GHND, len)?;
            let ptr = GlobalLock(handle)?;
            
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                ptr as *mut u8,
                len,
            );
            
            GlobalUnlock(handle).ok();
            
            SetClipboardData(CF_TEXT, handle).ok();
            
            CloseClipboard()?;
            
            Ok(())
        }
    }
}

// Helper traits for Windows API error handling
trait Win32Result<T> {
    fn ok(self) -> Result<(), T>;
}

impl Win32Result<()> for windows::Win32::Foundation::BOOL {
    fn ok(self) -> Result<(), ()> {
        if self.as_bool() {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        let service = TextSelectionService::new();
        assert!(service.get_selected_text_async().await.is_none());
    }

    #[tokio::test]
    async fn test_lock_mechanism() {
        let service = TextSelectionService::new();
        
        // First acquire should succeed
        let permit1 = service.lock.acquire().await;
        assert!(permit1.is_some());
        
        // Second acquire without wait should fail
        assert!(!service.lock.try_acquire().is_some());
        
        drop(permit1); // Release
        
        // Should be able to acquire again
        let permit2 = service.lock.try_acquire();
        assert!(permit2.is_some());
    }
}
