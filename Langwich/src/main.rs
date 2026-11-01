//! Langwich - Smart Persian ↔ English keyboard layout converter
//!
//! A Rust application for automatically converting text between Persian and English
//! keyboard layouts using a global hotkey trigger.
//!
//! # Features
//! - Automatic text conversion (Persian ↔ English)
//! - Global hotkey support (configurable)
//! - System tray icon with menu
//! - Settings persistence
//! - Windows startup integration
//!
//! # Example
//! ```
//! // Type "sghl" in English, press hotkey → "سلام" appears
//! // Type "سلام" in Persian, press hotkey → "sghl" appears
//! ```

use std::sync::{Arc, mpsc};
use std::thread;
use anyhow::{Context, Result};

// Import all modules
mod services;
mod models;
mod window;
mod ui;
mod helpers;

use services::{
    KeyboardLayoutConverter, TextSelectionService, HotkeyService, 
    TrayIconService, SettingsService, NotificationService, StartupService,
    TrayEvent
};
use models::AppSettings;
use window::HiddenHostWindow;
use helpers::native_methods::{combine_modifiers, VK_F10};

/// Application state shared across threads
struct AppState {
    /// Converter engine
    converter: Arc<std::sync::Mutex<KeyboardLayoutConverter>>,
    
    /// Text selection service
    text_service: Arc<TextSelectionService>,
    
    /// Hotkey service
    hotkey_service: Arc<HotkeyService>,
    
    /// Tray icon service  
    tray_service: Arc<TrayIconService>,
    
    /// Settings service
    settings_service: Arc<SettingsService>,
    
    /// Notification service
    notification_service: Arc<NotificationService>,
    
    /// Current settings
    settings: Arc<std::sync::Mutex<AppSettings>>,
}

impl AppState {
    fn new() -> Result<Self> {
        let converter = Arc::new(std::sync::Mutex::new(KeyboardLayoutConverter::new()));
        
        Ok(Self {
            converter,
            text_service: Arc::new(TextSelectionService::new()),
            hotkey_service: Arc::new(HotkeyService::new(9001)),
            tray_service: Arc::new(TrayIconService::new(1000, 0 as isize)),
            settings_service: Arc::new(SettingsService::new()),
            notification_service: Arc::new(NotificationService::new()),
            settings: Arc::new(std::sync::Mutex::new(AppSettings::default())),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦀 Langwich v1.2.0 (Rust Edition)");
    println!("Smart Persian ↔ English keyboard converter");
    println!();
    
    // Initialize services
    let app_state = AppState::new()?;
    
    // Load settings
    {
        let mut settings = app_state.settings.lock().unwrap();
        *settings = app_state.settings_service.load()?;
        println!("✅ Loaded {} settings", if (*settings).run_on_startup { "startup-enabled" } else { "default" });
    }
    
    // Initialize hidden host window (receives messages)
    let mut hidden_window = HiddenHostWindow::new();
    hidden_window.create()?;
    let hwnd = hidden_window.hwnd().unwrap_or(0);
    
    // Update tray icon with window handle
    let tray_hwnd = hwnd as isize;
    
    // Register global hotkey
    let settings = &*app_state.settings.lock().unwrap();
    
    if app_state.hotkey_service.register_hotkey(
        settings.hotkey_modifiers,
        settings.hotkey_key as u16
    ).is_ok() {
        println!("🎯 Hotkey registered: F10 (modifiers: 0x{:X})", settings.hotkey_modifiers);
    } else {
        eprintln!("⚠️  Failed to register hotkey, trying alternatives...");
        
        // Try alternative hotkeys
        for alt_vkey in [0x78, 0x7A, 0x7B] { // F9, F11, F12
            if app_state.hotkey_service.register_hotkey(0, alt_vkey).is_ok() {
                println!("✅ Alternative hotkey registered: F{}", alt_vkey - 0x70 + 9);
                break;
            }
        }
    }
    
    // Set up tray icon
    {
        let mut tray_service = app_state.tray_service.clone();
        tray_service.add_tray_icon("Langwich", "Persian ⇄ English Converter")?;
        
        // Setup event channel
        let (tx, rx) = mpsc::channel::<TrayEvent>();
        tray_service.set_event_sender(tx);
        
        thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                match event {
                    TrayEvent::ShowSettings => println!("📋 Show settings requested"),
                    TrayEvent::ExitRequested => println!("🚪 Exit requested"),
                }
            }
        });
    }
    
    // Show startup notification
    app_state.notification_service.show_startup(settings)?;
    
    // Main message loop
    println!("\n💤 Waiting for events (Ctrl+C to exit)...");
    
    // Keep running until terminated
    tokio::signal::ctrl_c().await?;
    
    println!("\n👋 Shutting down gracefully...");
    
    // Clean up
    app_state.hotkey_service.unregister_hotkey();
    
    Ok(())
}

/// Handles hotkey press event
async fn handle_hotkey_press(app_state: &AppState) -> Result<()> {
    // Lock both mutexes safely
    let converter = app_state.converter.lock().unwrap();
    let settings = app_state.settings.lock().unwrap();
    
    // Get selected text
    let selected_text = app_state.text_service.get_selected_text_async().await;
    
    if let Some(text) = selected_text {
        // Convert text
        let result = converter.convert(&text);
        
        // Only proceed if conversion changed something
        if !result.is_none() && result.converted_text != text {
            // Release locks before replacement
            drop(converter);
            drop(settings);
            
            // Replace text
            app_state.text_service.replace_selected_text_async(&result.converted_text).await.ok();
            
            // Show notification
            app_state.notification_service
                .show_conversion_success(&text, &result.converted_text, &settings)
                .ok();
            
            println!(
                "✓ Converted: '{}' → '{}'",
                text.trim(),
                result.converted_text.trim()
            );
        }
    }
    
    Ok(())
}
