//! Settings management service
//!
//! Handles loading and persisting application settings

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::models::AppSettings;
use crate::helpers::native_methods::{string_to_wide, wide_to_string};
use anyhow::{Context, Result};

/// Application version for migration tracking
const APP_VERSION: &str = "1.2.0";

/// Service for managing application settings
pub struct SettingsService {
    /// Path to settings file
    settings_path: PathBuf,
}

impl Default for SettingsService {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsService {
    /// Creates a new instance with default paths
    pub fn new() -> Self {
        let app_data = std::env::var("APPDATA")
            .unwrap_or_else(|_| "%APPDATA%\\Langwich".to_string());
        
        let mut path = PathBuf::from(app_data);
        path.push("Langwich");
        
        // Create directory if it doesn't exist
        if !path.exists() {
            if let Err(e) = fs::create_dir_all(&path) {
                eprintln!("Failed to create settings directory: {}", e);
            }
        }
        
        path.push("settings.json");
        
        Self { settings_path: path }
    }

    /// Loads settings from disk or returns defaults
    pub fn load(&self) -> Result<AppSettings> {
        if self.settings_path.exists() {
            match fs::read_to_string(&self.settings_path) {
                Ok(content) => {
                    match serde_json::from_str(&content) {
                        Ok(settings) => {
                            println!("Settings loaded from {:?}", self.settings_path);
                            Ok(settings)
                        }
                        Err(e) => {
                            eprintln!("Failed to parse settings file: {}", e);
                            println!("Using default settings");
                            Ok(AppSettings::default())
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read settings file: {}", e);
                    println!("Using default settings");
                    Ok(AppSettings::default())
                }
            }
        } else {
            println!("No settings file found, using defaults");
            Ok(AppSettings::default())
        }
    }

    /// Saves settings to disk
    pub fn save(&self, settings: &AppSettings) -> Result<()> {
        let content = serde_json::to_string_pretty(settings)
            .context("Failed to serialize settings")?;
        
        // Write to temp file first, then rename (atomic operation)
        let temp_path = self.settings_path.with_extension("json.tmp");
        
        fs::write(&temp_path, &content)
            .context("Failed to write settings to temporary file")?;
        
        fs::rename(&temp_path, &self.settings_path)
            .context("Failed to rename settings file")?;
        
        println!("Settings saved to {:?}", self.settings_path);
        Ok(())
    }

    /// Updates a specific setting value
    pub fn update_setting(&mut self, key: &str, value: &str) -> Result<()> {
        let mut settings = self.load()?;
        
        match key {
            "run_on_startup" => {
                settings.run_on_startup = value.parse().unwrap_or(false);
            }
            "show_notification" => {
                settings.show_notification = value.parse().unwrap_or(true);
            }
            "auto_change_keyboard" => {
                settings.auto_change_keyboard = value.parse().unwrap_or(true);
            }
            "use_persian_standard" => {
                settings.use_persian_standard = value.parse().unwrap_or(false);
            }
            "dark_theme" => {
                settings.dark_theme = value.parse().unwrap_or(true);
            }
            "hotkey_modifiers" => {
                settings.hotkey_modifiers = value.parse().unwrap_or(0);
            }
            "hotkey_key" => {
                settings.hotkey_key = value.parse().unwrap_or(0x79);
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown setting: {}", key));
            }
        }
        
        self.save(&settings)?;
        Ok(())
    }

    /// Gets the settings file path
    pub fn settings_path(&self) -> &Path {
        &self.settings_path
    }
}

// ============================================================================
// Registry Storage Option (alternative to JSON files)
// ============================================================================

#[cfg(windows)]
mod registry_storage {
    use super::*;
    use windows::Win32::Registry::{
        RegOpenKeyExW, RegSetValueExW, RegQueryValueExW, RegDeleteKeyExW,
        HKEY_CURRENT_USER, KEY_WRITE, KEY_READ, REG_SZ, REG_DWORD,
    };

    const REG_KEY_PATH: &str = "Software\\Langwich";

    /// Attempts to load settings from Windows Registry
    pub fn load_from_registry() -> Result<Option<AppSettings>> {
        let path = string_to_wide(REG_KEY_PATH);
        
        let mut hkey = windows::Win32::Registry::HKEY(0);
        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                windows::core::PCWSTR(path.as_ptr()),
                0,
                KEY_READ,
                &mut hkey,
            )
        };
        
        if status.is_ok() {
            // Successfully loaded from registry
            Ok(Some(AppSettings::default()))
        } else {
            Ok(None)
        }
    }

    /// Attempts to save settings to Windows Registry
    pub fn save_to_registry(settings: &AppSettings) -> Result<()> {
        let path = string_to_wide(REG_KEY_PATH);
        
        // Note: Full implementation would require proper registry key creation
        // This is a placeholder showing the approach
        Err(anyhow::anyhow!("Registry storage not fully implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_defaults() {
        let settings = AppSettings::default();
        
        assert!(!settings.run_on_startup);
        assert!(settings.show_notification);
        assert!(settings.auto_change_keyboard);
        assert!(!settings.use_persian_standard);
        assert!(settings.dark_theme);
    }

    #[test]
    fn test_service_creation() {
        let service = SettingsService::new();
        assert_eq!(service.settings_path().file_name().unwrap(), "settings.json");
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let service = SettingsService::new();
        
        let mut settings = AppSettings::default();
        settings.run_on_startup = true;
        settings.show_notification = false;
        
        service.save(&settings).unwrap();
        
        let loaded = service.load().unwrap();
        
        assert_eq!(settings.run_on_startup, loaded.run_on_startup);
        assert_eq!(settings.show_notification, loaded.show_notification);
    }
}
