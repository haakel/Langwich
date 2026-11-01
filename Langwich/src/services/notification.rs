//! Notification service for displaying user feedback
//!
//! Provides toast notifications and system tray balloon tips

use std::time::Duration;
use anyhow::{Context, Result};
use crate::models::AppSettings;

/// Service for managing user notifications
pub struct NotificationService;

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationService {
    /// Creates a new instance
    pub fn new() -> Self {
        Self
    }

    /// Shows a conversion success notification
    pub fn show_conversion_success(
        &self,
        original: &str,
        converted: &str,
        settings: &AppSettings,
    ) -> Result<()> {
        if !settings.show_notification {
            return Ok(());
        }
        
        let title = "Langwich";
        let message = format!(
            "Converted: {}\n→\n{}",
            original.trim(),
            converted.trim()
        );
        
        // In a full implementation, this would use Windows Toast API
        // For now, we'll log it
        println!("🔔 [Notification] {}", message);
        
        Ok(())
    }

    /// Shows an error notification
    pub fn show_error(&self, message: &str, settings: &AppSettings) -> Result<()> {
        if !settings.show_notification {
            return Ok(());
        }
        
        eprintln!("❌ [Error] {}", message);
        
        // In a full implementation, this would use Windows Error Notification
        Ok(())
    }

    /// Shows a hotkey conflict notification
    pub fn show_hotkey_conflict(
        &self,
        key: u16,
        modifiers: u32,
        settings: &AppSettings,
    ) -> Result<()> {
        if !settings.show_notification {
            return Ok(());
        }
        
        let key_name = vk_code_to_name(key);
        let msg = format!("Hotkey '{}' is already in use by another application", key_name);
        
        eprintln!("⚠️  [Conflict] {}", msg);
        
        Ok(())
    }

    /// Shows startup confirmation
    pub fn show_startup(&self, settings: &AppSettings) -> Result<()> {
        if settings.show_notification {
            println!("ℹ️  Langwich started successfully");
        }
        Ok(())
    }
}

/// Converts virtual key code to human-readable name
fn vk_code_to_name(vk: u16) -> String {
    match vk {
        0x08 => "Backspace".to_string(),
        0x09 => "Tab".to_string(),
        0x0D => "Enter".to_string(),
        0x1B => "Escape".to_string(),
        0x20 => "Space".to_string(),
        0x30..=0x39 => format!("'{}'", (vk as char).unwrap_or('?')),
        0x41..=0x5A => format!("{}", (vk as u8 as char).to_ascii_uppercase()),
        _ => format!("VK_{:#X}", vk),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vk_code_to_name() {
        assert_eq!(vk_code_to_name(0x41), "A");
        assert_eq!(vk_code_to_name(0x42), "B");
        assert_eq!(vk_code_to_name(0x30), "0");
        assert_eq!(vk_code_to_name(0x31), "1");
    }

    #[tokio::test]
    async fn test_notification_service() {
        let service = NotificationService::new();
        let settings = AppSettings::default();
        
        // These should not panic
        service.show_startup(&settings).unwrap();
        service.show_error("Test error", &settings).unwrap();
        service.show_hotkey_conflict(0x79, 0, &settings).unwrap();
    }
}
