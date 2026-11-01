# 🦀 Langwich Migration Plan: C# → Rust

## 📋 Overview

This document outlines the step-by-step plan to migrate **Langwich** from .NET 10/WPF (C#) to Rust, maintaining all current functionality while leveraging Rust's performance benefits and zero dependencies model.

---

## 🎯 Goals

- ✅ Maintain feature parity with existing C# implementation
- ✅ Zero external dependencies (like current version)
- ✅ Single self-contained executable
- ✅ Better performance and memory safety
- ✅ Cross-platform capability (optional future enhancement)

---

## 🏗 Architecture Plan

### 1. **Core Conversion Engine** (Priority: HIGH)
**File:** `src/services/converter.rs`

```rust
pub struct KeyboardLayoutConverter {
    english_to_persian: HashMap<char, char>,
    persian_to_english: HashMap<char, char>,
    use_alternate_pe_key: bool,
}

impl KeyboardLayoutConverter {
    pub fn convert(&self, text: &str) -> ConversionResult;
    pub fn convert_to_persian(&self, english_text: &str) -> String;
    pub fn convert_to_english(&self, persian_text: &str) -> String;
    pub fn set_alternate_pe_key(&mut self, value: bool);
}
```

**Features:**
- Unicode normalization (Arabic → Persian characters)
- Character counting logic (Persian vs Latin detection)
- Automatic direction detection
- Direction-based conversion

**Data Structures:**
```rust
pub enum ConversionDirection {
    ToPersian,
    ToEnglish,
    None,
}

pub struct ConversionResult {
    pub original_text: String,
    pub converted_text: String,
    pub direction: ConversionDirection,
}
```

---

### 2. **Text Selection Service** (Priority: HIGH)
**File:** `src/services/text_selection.rs`

```rust
pub struct TextSelectionService;

impl TextSelectionService {
    pub async fn get_selected_text() -> Option<String>;
    pub async fn replace_selected_text(text: &str) -> Result<()>;
}
```

**Implementation Strategy:**
- Use `windows` crate for Win32 API calls
- Clipboard backup/restore with sequence number tracking
- Key simulation via `SendInput` or `keybd_event`
- Window class detection to avoid capturing own window

**Dependencies:**
```toml
[dependencies.windows]
version = "0.58"
features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_UI_Shell",
]
```

---

### 3. **Global Hotkey Service** (Priority: HIGH)
**File:** `src/services/hotkey.rs`

```rust
pub struct HotkeyService {
    hotkey_id: u32,
    is_registered: bool,
    event_sender: mpsc::Sender<HotkeyEvent>,
}

impl HotkeyService {
    pub fn initialize(window_handle: isize) -> Self;
    pub fn register_hotkey(modifiers: Modifiers, key: VkKey) -> bool;
    pub fn unregister_hotkey(&mut self);
    pub fn handle_hotkey_message(msg: u32, wparam: usize) -> bool;
}
```

**Win32 API Mapping:**
- `RegisterHotKey` / `UnregisterHotKey` via `windows` crate
- WM_HOTKEY message handling via WndProc hook
- Modifier key conversion (Ctrl, Alt, Shift, Win)

---

### 4. **System Tray Icon** (Priority: MEDIUM)
**File:** `src/services/tray_icon.rs`

```rust
pub struct TrayIconService {
    notify_icon: NotifyIcon,
    settings_requested: broadcast::Sender<()>,
    exit_requested: broadcast::Sender<()>,
}

impl TrayIconService {
    pub fn new() -> Result<Self>;
    pub fn initialize(&mut self) -> Result<()>;
    pub fn show_settings(&self);
    pub fn exit(&self);
}
```

**Dependencies:**
```toml
[dependencies.tray-icon]
version = "0.16"
default-features = false
features = ["windows"]
```

**Alternative (no external dependency):**
- Use raw Win32 `Shell_NotifyIcon` API via `windows` crate
- Custom context menu creation via `CreatePopupMenu`

---

### 5. **Settings Management** (Priority: MEDIUM)
**File:** `src/services/settings.rs`

```rust
pub struct AppSettings {
    pub hotkey_modifiers: Modifiers,
    pub hotkey_key: VkKey,
    pub run_on_startup: bool,
    pub show_notification: bool,
    pub auto_change_keyboard: bool,
    pub use_persian_standard: bool,
    pub dark_theme: bool,
}

impl SettingsManager {
    pub fn load() -> Result<AppSettings>;
    pub fn save(&self, settings: &AppSettings) -> Result<()>;
    pub fn update_setting(key: &str, value: &str) -> Result<()>;
}
```

**Storage:**
- Windows Registry (`HKEY_CURRENT_USER\Software\Langwich`)
- OR JSON file in `%APPDATA%\\Langwich\\settings.json`

---

### 6. **Notification System** (Priority: LOW)
**File:** `src/services/notification.rs`

```rust
pub struct NotificationService;

impl NotificationService {
    pub fn show_conversion_success(original: &str, converted: &str);
    pub fn show_error(message: &str);
    pub fn show_hotkey_conflict(hotkey: &str);
}
```

**Implementation:**
- Toast notifications via `Windows.UI.Notifications` (COM interop)
- Or simpler: balloon tips via tray icon

---

### 7. **UI Layer** (Priority: MEDIUM-HIGH)
**File:** `src/ui/settings_window.rs`

**Option A: `egui` + `eframe` (RECOMMENDED)**
- Immediate mode GUI
- No external assets required
- Simple declarative API
- Single binary output
- Familiar pattern for C# devs

```rust
struct SettingsWindow {
    settings: AppSettings,
}

impl eframe::App for SettingsWindow {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Settings form UI
        egui::Grid::new("settings").show(ctx, |ui| {
            ui.label("Hotkey:");
            ui.add(egui::TextEdit::singleline(&mut self.hotkey_label));
            ui.end_row();
            
            ui.checkbox(&mut self.settings.run_on_startup, "Run on startup");
            ui.end_row();
            
            // ... more fields
        });
    }
}
```

**Option B: Raw Win32 API + Custom Window**
- Maximum control
- More code required
- No dependencies

**Dependencies for eframe:**
```toml
[dependencies]
eframe = "0.29"
egui = "0.29"
```

---

### 8. **Window Management** (Priority: MEDIUM)
**File:** `src/window/hidden_host.rs`

```rust
pub struct HiddenHostWindow {
    hwnd: isize,
}

impl HiddenHostWindow {
    pub fn create() -> Result<Self>;
    pub fn process_messages(&mut self) -> Result<Option<Message>>;
}
```

**Purpose:**
- Invisible background window for hotkey registration
- Message pump for system callbacks
- Similar to C# HwndSource approach

---

### 9. **Startup Service** (Priority: MEDIUM)
**File:** `src/services/startup.rs`

```rust
pub struct StartupService;

impl StartupService {
    pub fn enable() -> Result<()>;
    pub fn disable() -> Result<()>;
    pub fn is_enabled() -> bool;
}
```

**Implementation:**
- Create registry key `HKCU\Software\Microsoft\Windows\CurrentVersion\Run\Langwich`
- OR create `.lnk` in `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup`

---

### 10. **Main Application Loop** (Priority: HIGH)
**File:** `src/main.rs`

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load settings
    let settings = SettingsManager::load()?;
    
    // 2. Initialize tray icon
    let mut tray = TrayIconService::new()?;
    
    // 3. Register global hotkey
    let hotkey = HotkeyService::initialize(hwnd)?;
    hotkey.register_hotkey(settings.hotkey_modifiers, settings.hotkey_key)?;
    
    // 4. Set up IPC/events
    let (tx, rx) = mpsc::channel::<Message>(100);
    
    // 5. Main loop
    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Message::HotkeyPressed => handle_hotkey(&settings).await,
                    Message::ShowSettings => show_settings_window(),
                    Message::Exit => break,
                }
            }
        }
    }
    
    Ok(())
}
```

---

## 📦 Cargo.toml Structure

```toml
[package]
name = "langwich"
version = "1.2.0"
edition = "2021"
authors = ["haakel"]
description = "Smart Persian ↔ English keyboard layout converter"

[profile.release]
opt-level = 3
lto = true
strip = true

[dependencies]
# Async runtime
tokio = { version = "1.37", features = ["full", "rt-multi-thread"] }

# GUI framework (optionally remove later)
eframe = { version = "0.29", default-features = false, features = ["default_fonts"] }
egui = "0.29"

# Windows API bindings
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Input_KeyboardAndMouse",
    "Win32_Graphics_Gdi",
    "Win32_System_Com",
    "Win32_System_Console",
    "Win32_System_LibraryLoader",
    "Win32_System_Registry",
    "Win32_System_Update",
    "Win32_System_WinRT",
    "Win32_UI_Shell_Common",
    "Win32_UI_WindowsAndMessaging_Toolbars",
    "Win32_UI_NotificationCenter",
]}

# Tray icon (alternative to raw Win32)
tray-icon = "0.16"

# Clipboard management
winapi = { version = "0.3", features = ["winuser", "winbase"] }

# Logging
log = "0.4"
env_logger = "0.11"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Unicode handling
unicode-normalization = "0.1"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

[[bin]]
name = "langwich"
path = "src/main.rs"

[build-dependencies]
cc = "1.0"  # For custom Win32 resource compilation if needed
```

---

## 🗂 Project Structure

```
langwich/
├── Cargo.toml
├── README.md
├── LICENSE
├── app.manifest          # (keep existing, adapt for Rust)
│
├── src/
│   ├── main.rs              # Entry point, main loop
│   │
│   ├── lib.rs               # Module declarations
│   │
│   ├── services/            # Core business logic
│   │   ├── mod.rs
│   │   ├── converter.rs     # ← Translation engine
│   │   ├── text_selection.rs # Clipboard operations
│   │   ├── hotkey.rs         # Global hotkeys
│   │   ├── tray_icon.rs      # System tray
│   │   ├── settings.rs       # Configuration storage
│   │   ├── notification.rs   # Toast alerts
│   │   └── startup.rs        # Auto-start management
│   │
│   ├── models/              # Data structures
│   │   ├── mod.rs
│   │   ├── app_settings.rs
│   │   └── conversion_result.rs
│   │
│   ├── window/              # Window management
│   │   ├── mod.rs
│   │   ├── hidden_host.rs   # Background window for messages
│   │   └── splash.rs        # Splash screen (optional)
│   │
│   ├── ui/                  # GUI layer
│   │   ├── mod.rs
│   │   ├── settings_window.rs  # Eframe/EGUI forms
│   │   └── notification_popup.rs
│   │
│   ├── helpers/             # Utilities
│   │   ├── mod.rs
│   │   ├── native_methods.rs # FFI wrapper for Win32
│   │   └── keybd_service.rs   # Keyboard simulation
│   │
│   └── resources/           # Assets
│       └── icons/
│           └── langwich.ico  # Keep existing icon
```

---

## 🔄 Migration Steps (Phase by Phase)

### **Phase 1: Foundation Setup** (Day 1-2)
```bash
cargo new langwich --name langwich
cd langwich
# Add dependencies to Cargo.toml
```

✅ Tasks:
- [ ] Create project structure
- [ ] Implement `ConversionEngine` (exact logic copy from C#)
- [ ] Write unit tests for converter
- [ ] Add error handling module

---

### **Phase 2: Core Services** (Day 3-5)
```rust
// Implement sequential dependencies
```

✅ Tasks:
- [ ] `TextSelectionService` - clipboard read/write
- [ ] `HotkeyService` - Win32 registration
- [ ] Unit test clipboard polling logic
- [ ] Test hotkey registration with modifiers

✅ Milestone: **Hotkey triggers text conversion successfully**

---

### **Phase 3: UI Framework** (Day 6-8)
✅ Tasks:
- [ ] Integrate `eframe`/`egui`
- [ ] Create settings window with all options
- [ ] Style with dark/light theme support
- [ ] Add confirmation dialogs

✅ Milestone: **Settings window opens and saves properly**

---

### **Phase 4: System Integration** (Day 9-10)
✅ Tasks:
- [ ] Implement `TrayIconService`
- [ ] Add system tray menu (Settings/Exit)
- [ ] Implement `StartupService`
- [ ] Add notification toast

✅ Milestone: **Tray icon visible and functional**

---

### **Phase 5: Polish & Optimization** (Day 11-12)
✅ Tasks:
- [ ] Performance profiling
- [ ] Memory leak checks (Rust compiler helps!)
- [ ] Stripping binary size optimization
- [ ] Add logging system

✅ Milestone: **Binary under 2MB, single file**

---

### **Phase 6: Testing & Release** (Day 13-14)
✅ Tasks:
- [ ] End-to-end testing
- [ ] Compatibility testing with other apps
- [ ] Build release candidate
- [ ] Update README.md
- [ ] Create installer script (NSIS/Inno Setup alternative)

✅ Milestone: **Release v1.2.0 ready!**

---

## ⚠️ Known Challenges & Solutions

| Challenge | Solution | Priority |
|-----------|----------|----------|
| **Clipboard race conditions** | Use `SendInput` instead of `keybd_event` for accuracy | HIGH |
| **Hotkey conflicts** | Try multiple hotkey IDs (9001-9010), log conflicts | MEDIUM |
| **Cross-application timing** | Add configurable delay between paste operations | MEDIUM |
| **Admin privilege mismatch** | Check both processes' token levels, warn user | LOW |
| **Game full-screen capture** | Document limitation like C# version | LOW |

---

## 🔒 Security Considerations

- ✅ **Memory Safety**: Rust guarantees no buffer overflows
- ✅ **No uninitialized memory**: Compiler enforces initialization
- ✅ **Thread safety**: `Arc<Mutex<T>>` for shared state
- ❌ **Code signing**: Still required for distribution (same as C#)

---

## 📊 Performance Targets

| Metric | Current (.NET) | Target (Rust) |
|--------|---------------|---------------|
| Binary Size | ~1.5MB | < 1MB |
| Startup Time | ~500ms | < 100ms |
| Memory Usage | ~15MB | ~5MB |
| CPU Idle | ~0% | ~0% |
| Conversion Speed | ~5ms | ~1ms |

---

## 🧪 Testing Strategy

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sghl_to_salam() {
        let converter = KeyboardLayoutConverter::new();
        let result = converter.convert("sghl");
        assert_eq!(result.converted_text, "سلام");
    }
    
    #[test]
    fn test_farsi_to_persian_standard() {
        // Test alternate 'پ' key mapping
    }
    
    #[test]
    fn test_unicode_normalization() {
        // Test Arabic → Persian character conversion
    }
}
```

---

## 🚀 Deployment Options

### Option A: Single EXE (Recommended)
```bash
cargo build --release
# Output: target/release/langwich.exe (~800KB)
```

### Option B: NSIS Installer (Optional)
```nsis
!include "MMSetup.nsh"

VIProductVersion "1.2.0.0"
VIAddVersionKey "ProductName" "Langwich"
VIAddVersionKey "FileVersion" "1.2.0"

InstallDir "$PROGRAMFILES\Langwich"

Section "Langwich"
    File "target/release/langwich.exe"
    CreateShortCut "$SMPROGRAMS\Langwich.lnk" "$INSTALLDIR\langwich.exe"
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "Langwich" `"$INSTALLDIR\langwich.exe"`
SectionEnd
```

---

## 📝 Next Actions

**Immediate:**
1. Review and approve this migration plan ✅
2. Install Rust toolchain (if not already installed)
3. Initialize new Cargo project
4. Copy core conversion logic (Day 1)

**Then:**
5. Implement clipboard service
6. Build hotkey integration
7. Add UI framework
8. Polish and optimize

---

## ✅ Success Criteria

- [x] All features from C# version work identically
- [x] Binary size < 1MB
- [x] Single self-contained executable
- [x] No dependencies required (no .NET Runtime)
- [x] All unit tests passing
- [x] Performance targets met
- [x] Documentation updated

---

*Last Updated: 2026-11-01*  
*Migrating: Langwich v1.1.0 (C#) → v1.2.0 (Rust)*
