/// Application settings that can be persisted
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// Hotkey key for triggering conversion
    pub hotkey_key: u32, // VK_KEY virtual key code
    
    /// Hotkey modifiers (MOD_ALT, MOD_CONTROL, etc.)
    pub hotkey_modifiers: u32,
    
    /// Whether to run on Windows startup
    pub run_on_startup: bool,
    
    /// Whether to show notifications
    pub show_notification: bool,
    
    /// Whether to automatically change keyboard layout after conversion
    pub auto_change_keyboard: bool,
    
    /// Whether to use Persian standard ('پ' on backslash key)
    pub use_persian_standard: bool,
    
    /// Whether to use dark theme
    pub dark_theme: bool,
    
    /// Whether splash screen has been shown
    pub has_shown_splash: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hotkey_key: 0x79, // F10 virtual key
            hotkey_modifiers: 0,
            run_on_startup: false,
            show_notification: true,
            auto_change_keyboard: true,
            use_persian_standard: false,
            dark_theme: true,
            has_shown_splash: false,
        }
    }
}
