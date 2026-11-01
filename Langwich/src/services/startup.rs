//! Startup service for automatic Windows launch
//!
//! Manages application startup registry keys or Start Menu shortcuts

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// Service for managing application startup configuration
pub struct StartupService;

impl Default for StartupService {
    fn default() -> Self {
        Self::new()
    }
}

impl StartupService {
    /// Creates a new instance
    pub fn new() -> Self {
        Self
    }

    /// Enables automatic startup on Windows login
    pub fn enable() -> Result<()> {
        let exe_path = std::env::current_exe()
            .context("Failed to get current executable path")?;
        
        #[cfg(windows)]
        {
            use windows::Win32::Registry::{RegOpenKeyExW, RegSetValueExW, KEY_WRITE};
            use crate::helpers::native_methods::string_to_wide;
            
            const REG_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
            const REG_VALUE: &str = "Langwich";
            
            let key_path = string_to_wide(REG_KEY);
            let value_path = string_to_wide(REG_VALUE);
            
            let mut hkey = windows::Win32::Registry::HKEY(0);
            
            let status = unsafe {
                RegOpenKeyExW(
                    windows::Win32::Registry::HKEY_CURRENT_USER,
                    windows::core::PCWSTR(key_path.as_ptr()),
                    0,
                    KEY_WRITE,
                    &mut hkey,
                )
            };
            
            if status.is_ok() {
                let exe_path_str = exe_path.to_string_lossy().to_string();
                let exe_path_wide = string_to_wide(&exe_path_str);
                
                let status = unsafe {
                    RegSetValueExW(
                        hkey,
                        windows::core::PCWSTR(value_path.as_ptr()),
                        0,
                        1, // REG_SZ
                        Some(exe_path_wide.as_ptr() as *const u8),
                        (exe_path_wide.len() + 1) * 2, // Multiply by 2 for UTF-16
                    )
                };
                
                if status.is_ok() {
                    println!("✅ Langwich will start on Windows login");
                    return Ok(());
                }
            }
        }
        
        // Fallback: Try creating Start Menu shortcut
        let start_menu = get_startup_folder()?;
        
        fs::create_dir_all(&start_menu)?;
        
        // Note: Creating actual .lnk files requires COM objects in Rust
        // This is a simplified placeholder showing the approach
        println!("ℹ️  Would create startup shortcut at: {:?}", start_menu);
        
        Err(anyhow::anyhow!(
            "Startup enabling completed via fallback method"
        ))
    }

    /// Disables automatic startup
    pub fn disable() -> Result<()> {
        #[cfg(windows)]
        {
            use windows::Win32::Registry::{RegOpenKeyExW, RegDeleteValueW, KEY_WRITE};
            use crate::helpers::native_methods::string_to_wide;
            
            const REG_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
            const REG_VALUE: &str = "Langwich";
            
            let key_path = string_to_wide(REG_KEY);
            let value_path = string_to_wide(REG_VALUE);
            
            let mut hkey = windows::Win32::Registry::HKEY(0);
            
            let status = unsafe {
                RegOpenKeyExW(
                    windows::Win32::Registry::HKEY_CURRENT_USER,
                    windows::core::PCWSTR(key_path.as_ptr()),
                    0,
                    KEY_WRITE,
                    &mut hkey,
                )
            };
            
            if status.is_ok() {
                let status = unsafe {
                    RegDeleteValueW(
                        hkey,
                        windows::core::PCWSTR(value_path.as_ptr()),
                    )
                };
                
                if status.is_ok() {
                    println!("🚫 Langwich disabled from Windows startup");
                    return Ok(());
                }
            }
        }
        
        Ok(())
    }

    /// Checks if application is configured to start on Windows login
    pub fn is_enabled() -> bool {
        #[cfg(windows)]
        {
            use windows::Win32::Registry::{RegOpenKeyExW, RegQueryValueExW, KEY_READ};
            use crate::helpers::native_methods::string_to_wide;
            
            const REG_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
            const REG_VALUE: &str = "Langwich";
            
            let key_path = string_to_wide(REG_KEY);
            let value_path = string_to_wide(REG_VALUE);
            
            let mut hkey = windows::Win32::Registry::HKEY(0);
            
            let status = unsafe {
                RegOpenKeyExW(
                    windows::Win32::Registry::HKEY_CURRENT_USER,
                    windows::core::PCWSTR(key_path.as_ptr()),
                    0,
                    KEY_READ,
                    &mut hkey,
                )
            };
            
            if !status.is_ok() {
                return false;
            }
            
            let mut data_type = 0;
            let mut buffer: Vec<u8> = vec![0; 260];
            let mut size = buffer.len() as u32;
            
            let status = unsafe {
                RegQueryValueExW(
                    hkey,
                    windows::core::PCWSTR(value_path.as_ptr()),
                    None,
                    Some(&mut data_type),
                    Some(buffer.as_mut_ptr()),
                    Some(&mut size),
                )
            };
            
            status.is_ok()
        }
        
        #[cfg(not(windows))]
        {
            false // Not implemented for non-Windows platforms
        }
    }
}

/// Gets the user's startup folder path
fn get_startup_folder() -> Result<PathBuf> {
    let app_data = std::env::var("APPDATA")
        .unwrap_or_else(|_| "%APPDATA%\\Microsoft\\Windows\\Start Menu\\Programs\\Startup");
    
    let mut path = PathBuf::from(app_data);
    path.push("Startup");
    
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_enabled_by_default() {
        // Should be false initially
        assert!(!StartupService::is_enabled());
    }
}
