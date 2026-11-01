# Langwich — Persian ↔ English Keyboard Converter (Rust Edition)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/haakel/langwich-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/haakel/langwich-rs/actions)

> **زبان / Language:** [🇮🇷 فارسی](#راهنمای-فارسی) | [🇬🇧 English](#english-guide)

---

# راهنمای فارسی

## معرفی
**Langwich** یک ConvertorKeyboard هوشمند است که متون را به صورت خودکار بین فارسی و انگلیسی ترجمه می‌کند، بر اساس لایه کلیدهای فارسی مایکروسافت. این پروژه به زبان **Rust** نوشته شده است تا بهترین عملکرد و کمترین وابستگی‌ها را داشته باشد.

## ویژگی‌ها
- ✨ **ترجمه خودکار**: تایپ «sghl» به انگلیسی → تولید «سلام» به فارسی
- ⚡ **کلید سراسری**: فعال کردن ترجمه از هر برنامه (پیش‌فرض: F10)
- 🔧 **تنظیمات قابل تغییر**: شخصی‌سازی کلید، راه‌اندازی با ویندوز و ترجیحات
- 💡 **نماد localized در 트ای**: دسترسی آسان به تنظیمات و وضعیت
- 🚀 **بدون وابستگی**: فایل EXE proprio و ایزوله
- 🔒 **ایمن به جهة حافظه**: رست از przepełnienia بافر و نشت‌های حافظه جلوگیری می‌کند
- ⚙️ **پشتیبانی از استاندارد فارسی**: امکان استفاده از لایه استاندارد فارسی ('پ' روی کلید \)

## نیازمندی‌ها
- ویندوز 7 یا نسخه‌های جدیدتر
- نیاز به رنتای姆 اضافی ندارد (EXE ایزوله)

## نصب
### دریافت نسخه آماده
آخرین نسخه را از [صفحه Released](https://github.com/haakel/langwich/releases) دریافت کنید:
```bash
curl -LO https://github.com/haakel/langwich/releases/download/v1.2.0/langwich-x64.zip
tar -xzf langwich-x64.zip
```

### ساخت از کد منبع
```bash
# پیش‌نیازها:
# - رست ۱.۷۰+ (از https://rustup.rs/ نصب شود)

git clone https://github.com/haakel/langwich-rs.git
cd langwich-rs

cargo build --release
# فایل اجرایی در: target/release/langwich.exe
```

## نحوه استفاده
1. **اجرا** `langwich.exe`
2. **انتخاب متن** در هر برنامه (مرورگر، ویرایشگر و غیره)
3. **فشار دادن کلید heat** (پیش‌فرض: F10)
4. متن به صورت خودکار با نسخه ترجمه شده جایگزین می‌شود

## مثال‌های سریع
| ورودی (انگلیسی) | کلید فشرده | خروجی (فارسی) |
|----------------|-------------|----------------|
| `sghl` | F10 | `سلام` |
| `khbay` | F10 | `خوبی` |
| `mamnoo` | F10 | `ممنون` |

| ورودی (فارسی) | کلید فشرده | خروجی (انگلیسی) |
|--------------|-------------|------------------|
| `سلام` | F10 | `sghl` |
| `خوبی` | F10 | `khbay` |
| `ممنون` | F10 | `mamnoo` |

## پیکربندی
### تنظیم کلید
پیش‌فرض: **بدون مودیفایر + F10**

ترکیبات دیگر:
- `Alt + F10`
- `Ctrl + Shift + P`
- هر ترکیب دلخواه

دسترسی به تنظیمات:
1. کلیک راست روی نماد 트ای سیستمی
2. کلیک روی دکمه «Settings...»
3. تغییر پیکربندی کلید

### تنظیمات برنامه
- **اجرا هنگام شروع ویندوز**: شروع خودکار هنگام بوت شدن ویندوز
- **نمایش اعلان‌ها**: نمایش اعلان هنگام تبدیل موفق
- **تغییر خودکار لایه کلید**: تغییر لایه کلید سیستم پس از تبدیل
- **استفاده از استاندارد فارسی**: فعال‌سازی لایه 'پ' روی کلید بک‌Slash
- **مایه تیره**: تغییر ظاهر antarmuk

## توسعه
### ساخت و تست
```bash
cargo build              # نسخه دیباگ (~۵ مگابایت)
cargo build --release    # نسخه رلیز بهینه شده (~۸۰۰ کیلوبایت)
cargo test               # اجرای همه تست‌ها
```

### ساختار پروژه
```text
langwich/
├── Cargo.toml             # وابستگی‌ها
├── src/
│   ├── main.rs            # نقطه ورود و حلقه پیام
│   ├── lib.rs             # deklar� ماژول‌ها
│   ├── services/          # عملکرد اصلی
│   ├── models/            # ساختاری داده
│   ├── window/            # مدیریت پنجره
│   ├── ui/                # اجزای رابط کاربری
│   └── helpers/           # ابزار کمکی
```

## مجوز
مجوز MIT - فایل [LICENSE](../LICENSE) را ببینید

## مشارکت
مشارکت‌ها خوش‌آمدند! لطفا Pull Request بفرستید.

1. Fork ریپازیتوری کنید
2. شاخه ویژگی بسازید (`git checkout -b feature/amazing-feature`)
3. کامیت کنید (`git commit -m 'Add amazing feature'`)
4. push کنید (`git push origin feature/amazing-feature`)
5. Pull Request باز کنید

---

# English Guide

## Introduction
**Langwich** is a smart keyboard layout converter that automatically translates text between Persian and English using the Microsoft Persian keyboard layout. Built in **Rust** for maximum performance and zero dependencies.

## Features
- ✨ **Automatic Translation**: Type "sghl" in English → get "سلام" in Persian
- ⚡ **Global Hotkey**: Trigger conversion from any application (default: F10)
- 🔧 **Configurable Settings**: Customize hotkey, startup behavior, and preferences
- 💡 **System Tray Icon**: Easy access to settings and status monitoring
- 🚀 **Zero Dependencies**: Single self-contained executable
- 🔒 **Memory Safe**: Rust guarantees no buffer overflows or memory leaks
- ⚙️ **Persian Standard Support**: Option to use Persian standard layout ('پ' on \\ key)

## Requirements
- Windows 7 or later
- No additional runtime required (standalone EXE)

## Installation
### Download Pre-built Release
Download the latest release from [Releases page](https://github.com/haakel/langwich/releases):

```bash
# Example download
curl -LO https://github.com/haakel/langwich/releases/download/v1.2.0/langwich-x64.zip
tar -xzf langwich-x64.zip
```

### Build from Source
```bash
# Prerequisites:
# - Rust 1.70+ (install from https://rustup.rs/)

# Clone repository
git clone https://github.com/haakel/langwich-rs.git
cd langwich-rs

# Build release binary
cargo build --release

# Binary location: target/release/langwich.exe
```

## Usage
1. **Launch** `langwich.exe`
2. **Select text** in any application (browser, editor, etc.)
3. **Press hotkey** (default: F10) to convert
4. Text is automatically replaced with translated version

### Quick Examples
| Input (English) | Hotkey Pressed | Output (Persian) |
|----------------|----------------|------------------|
| `sghl`         | F10            | `سلام`          |
| `khbay`        | F10            | `خوبی`           |
| `mamnoo`       | F10            | `ممنون`          |

| Input (Persian) | Hotkey Pressed | Output (English) |
|----------------|----------------|------------------|
| `سلام`        | F10            | `sghl`          |
| `خوبی`          | F10            | `khbay`           |
| `ممنون`          | F10            | `mamnoo`          |

## Configuration
### Hotkey Setup
Default: **No modifiers + F10**

Other combinations:
- `Alt + F10`
- `Ctrl + Shift + P`
- Any custom combination

Access settings via:
1. Right-click system tray icon
2. Click "Settings..." button
3. Modify hotkey configuration

### Application Settings
- **Run on Windows startup**: Auto-start when Windows boots
- **Show notifications**: Display conversion success toast
- **Auto-switch keyboard**: Change system keyboard layout after conversion
- **Use Persian Standard**: Enable 'پ' on backslash key layout
- **Dark theme**: Toggle UI appearance

## Development
### Project Structure
```
langwich/
├── src/
│   ├── main.rs              # Entry point & message loop
│   ├── lib.rs               # Module declarations
│   │
│   ├── services/            # Core functionality
│   │   ├── converter.rs     # Text conversion logic
│   │   ├── hotkey.rs        # Global hotkey registration
│   │   ├── notification.rs  # System tray
│   │   ├── settings.rs      # Configuration
│   │   ├── startup.rs       # Auto-start
│   │   ├── text_selection.rs
│   │   └── tray_icon.rs
│   ├── models/              # Data structures
│   │   ├── app_settings.rs
│   │   ├── conversion_direction.rs
│   │   └── conversion_result.rs
│   ├── window/              # Window management
│   │   └── hidden_host.rs   # Message window
│   ├── ui/                  # GUI components
│   │   └── settings_window.rs # Settings dialog
│   └── helpers/             # Utilities
│       ├── native_methods.rs # Win32 wrappers
│       └── keybd_service.rs  # Keyboard simulation
│
└── Cargo.toml               # Dependencies
```

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_sghl_to_salam

# Run with output
cargo test -- --nocapture
```

### Building Debug Version
```bash
cargo build              # Debug build (~5MB)
```

### Building Release Version
```bash
cargo build --release    # Optimized release (~800KB)
```

## Performance Targets
| Metric       | C# Version | Rust Target |
|--------------|------------|-------------|
| Binary Size  | ~1.5MB     | < 1MB       |
| Startup Time | ~500ms     | < 100ms     |
| Memory Usage | ~15MB      | ~5MB        |
| Conversion   | ~5ms       | ~1ms        |

## Technical Details
### Character Mapping
Based on **Microsoft Persian** keyboard layout:

| QWERTY | Persian | Key Position |
|--------|---------|--------------|
| q  | ض   | Top-left      |
| w  | ص   | Top-row       |
| e  | ث   | Top-row       |
| s  | س   | Home-row      |
| d  | ی   | Home-row      |
| f  | ب   | Home-row      |
| g  | ل   | Home-row      |
| m  | پ   | Bottom-row (or \\) |

### Unicode Normalization
Automatically handles Arabic/Persian character differences:
- Arabic ك → Persian ک
- Arabic ي → Persian ی  
- Arabic ة → Persian ه
- And more...

## Security
- ✅ **Memory Safe**: Zero undefined behavior
- ✅ **No Heap Overflow**: Compiler-enforced bounds checking
- ✅ **Thread Safe**: Proper synchronization primitives
- ✅ **No Code Injection**: Static typing guarantees

## License
MIT License - See [LICENSE](../LICENSE) file for details

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Known Issues
- ❌ Full-screen game capture may not work (same limitation as C# version)
- ❌ Admin privilege mismatch warnings possible
- ❌ Some applications may block clipboard access

## Changelog
### v1.2.0 (Rust Edition)
- ✨ Initial Rust implementation
- ✨ Zero-dependency standalone executable
- ✨ Improved performance and stability
- ✨ Modern GUI with egui

### Upcoming
- 🔄 Installer creation (NSIS)
- 🔄 Code signing integration
- 🔄 Cross-platform support (Linux/macOS)

---

Built with ❤️ in Rust | Originally developed in C#, now migrated for better performance and portability

*Last updated: November 1, 2026'
