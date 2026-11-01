//! Settings window using egui/eframe
//!
//! Provides a simple settings dialog using immediate mode GUI

use std::sync::Mutex;
use egui::{CentralPanel, Context, PanelShape, RichText, Ui, Vec2};
use eframe::egui;
use crate::models::AppSettings;

/// Settings window data structure
pub struct SettingsWindow {
    /// Current application settings
    pub settings: AppSettings,
    
    /// Whether hotkey change is active
    pub hotkey_change_mode: bool,
    
    /// Temporary hotkey value for preview
    pub hotkey_preview: String,
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsWindow {
    /// Creates a new settings window with default values
    pub fn new() -> Self {
        Self {
            settings: AppSettings::default(),
            hotkey_change_mode: false,
            hotkey_preview: String::from("F10"),
        }
    }

    /// Loads settings from storage
    pub fn load_settings(&mut self, service: &crate::services::SettingsService) -> Result<(), String> {
        self.settings = service.load().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Saves current settings
    pub fn save_settings(&self, service: &crate::services::SettingsService) -> Result<(), String> {
        service.save(&self.settings).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Starts hotkey change mode
    pub fn start_hotkey_change(&mut self) {
        self.hotkey_change_mode = true;
        self.hotkey_preview = format!("{}+{}", 
            self.modifier_str(),
            key_code_to_str(self.settings.hotkey_key)
        );
    }

    /// Applies selected hotkey
    pub fn apply_hotkey(&mut self, modifiers: u32, vkey: u32) {
        self.settings.hotkey_modifiers = modifiers;
        self.settings.hotkey_key = vkey;
        self.hotkey_change_mode = false;
        self.hotkey_preview = format!("{}+{}", 
            self.modifier_str(),
            key_code_to_str(vkey)
        );
    }

    /// Gets modifier string representation
    fn modifier_str(&self) -> String {
        let mut parts = Vec::new();
        
        if (self.settings.hotkey_modifiers & 1) != 0 {
            parts.push("Alt");
        }
        if (self.settings.hotkey_modifiers & 2) != 0 {
            parts.push("Ctrl");
        }
        if (self.settings.hotkey_modifiers & 4) != 0 {
            parts.push("Shift");
        }
        if (self.settings.hotkey_modifiers & 8) != 0 {
            parts.push("Win");
        }
        
        if parts.is_empty() {
            "None".to_string()
        } else {
            parts.join("+")
        }
    }
}

/// Renders the settings panel
pub fn render_settings_panel(
    ctx: &Context,
    app_data: &mut SettingsWindow,
    on_save: &dyn Fn(&AppSettings),
    on_close: &dyn Fn(),
) {
    CentralPanel::default()
        .frame(
            egui::Frame::none()
                .fill(ctx.style().visuals.panel_fill)
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
        )
        .show(ctx, |ui| {
            // Header
            ui.heading(RichText::new("Langwich Settings")
                .size(20.0)
                .bold());
            
            ui.add_space(16.0);
            
            // Hotkey section
            ui.label("Conversion Hotkey:");
            
            ui.horizontal(|ui| {
                if app_data.hotkey_change_mode {
                    ui.label(format!("Press any key: {}", app_data.hotkey_preview));
                    
                    // Keyboard listeners would go here in a real implementation
                    ui.add_space(8.0);
                    ui.button("Cancel").on_hover_text("Press Escape");
                } else {
                    ui.label(format!(
                        "{}",
                        RichText::new(format!(
                            "{} + {}",
                            app_data.modifier_str(),
                            key_code_to_str(app_data.settings.hotkey_key)
                        ))
                        .strong()
                    ));
                    
                    if ui.button("Change").clicked() {
                        app_data.start_hotkey_change();
                    }
                }
            });
            
            ui.add_space(16.0);
            
            // Checkbox options
            ui.add(egui::Checkbox::new(
                &mut app_data.settings.run_on_startup,
                "Run on Windows startup",
            ));
            
            ui.add(egui::Checkbox::new(
                &mut app_data.settings.show_notification,
                "Show conversion notifications",
            ));
            
            ui.add(egui::Checkbox::new(
                &mut app_data.settings.auto_change_keyboard,
                "Automatically switch keyboard layout",
            ));
            
            ui.add(egui::Checkbox::new(
                &mut app_data.settings.use_persian_standard,
                "Use Persian Standard ('پ' on \\)",
            ));
            
            ui.add(egui::Checkbox::new(
                &mut app_data.settings.dark_theme,
                "Dark theme",
            ));
            
            ui.add_space(24.0);
            
            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    on_save(&app_data.settings);
                    // Note: Close button should trigger close event
                }
                
                if ui.button("Close").clicked() {
                    on_close();
                }
            });
        });
}

/// Converts virtual key code to name string
fn key_code_to_str(code: u32) -> String {
    match code {
        0x08 => "Backspace".to_string(),
        0x09 => "Tab".to_string(),
        0x0D => "Enter".to_string(),
        0x1B => "Esc".to_string(),
        0x20 => "Space".to_string(),
        0x30..=0x39 => format!("{}", code),
        0x41..=0x5A => format!("{}", (code as u8 as char).to_ascii_uppercase()),
        0xF1..=0xFE => format!("F{}", code - 0x70),
        _ => format!("{}", code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_window_creation() {
        let window = SettingsWindow::new();
        assert!(!window.settings.run_on_startup);
        assert_eq!(window.hotkey_preview, "None+F10");
    }

    #[test]
    fn test_modifier_string() {
        let mut window = SettingsWindow::new();
        assert_eq!(window.modifier_str(), "None");
        
        window.settings.hotkey_modifiers = 1; // Alt
        assert_eq!(window.modifier_str(), "Alt");
        
        window.settings.hotkey_modifiers = 2; // Control
        assert_eq!(window.modifier_str(), "Ctrl");
        
        window.settings.hotkey_modifiers = 3; // Alt + Ctrl
        assert_eq!(window.modifier_str(), "Alt+Ctrl");
    }
}
