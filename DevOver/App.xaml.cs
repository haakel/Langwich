using System.Windows;
using DevOver.Helpers;
using DevOver.Services;
using DevOver.Services.Interfaces;
using DevOver.ViewModels;
using DevOver.Views;

namespace DevOver;

/// <summary>
/// نقطه‌ی شروع برنامه. تمام سرویس‌ها را ایجاد و به هم متصل می‌کند (Composition Root).
/// برنامه هیچ پنجره‌ی اصلی ندارد؛ در سینی سیستم (System Tray) اجرا می‌شود.
/// </summary>
public partial class App : Application
{
    // ---- سرویس‌ها ----
    private ISettingsService?  _settingsService;
    private IHotkeyService?    _hotkeyService;
    private ITrayIconService?  _trayIconService;
    private ITextSelectionService? _textSelectionService;
    private IKeyboardLayoutConverterService? _converterService;
    private INotificationService? _notificationService;
    private IStartupService?   _startupService;

    private HiddenHostWindow? _hiddenHost;
    private System.Threading.Mutex? _mutex;

    // ---- تنظیمات ----
    private System.Windows.Input.ModifierKeys _currentModifiers;
    private System.Windows.Input.Key          _currentKey;

    protected override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);

        // ۱. تنها یک نمونه از برنامه اجرا شود
        _mutex = new System.Threading.Mutex(
            initiallyOwned: true,
            name: "Langwich_Global_SingleInstance_Mutex_8F3A1C",
            out bool createdNew);

        if (!createdNew)
        {
            MessageBox.Show(
                "Langwich در حال اجرا است.\nآیکون آن را در سینی سیستم (کنار ساعت) پیدا کنید.",
                "Langwich",
                MessageBoxButton.OK,
                MessageBoxImage.Information);
            Shutdown();
            return;
        }

        // ۲. ایجاد سرویس‌ها (بدون DI Container – ساده و قابل فهم)
        _settingsService      = new SettingsService();
        _converterService     = new KeyboardLayoutConverterService();
        _textSelectionService = new TextSelectionService();
        _notificationService  = new NotificationService();
        _startupService       = new StartupService();

        // ۳. بارگذاری تنظیمات ذخیره‌شده
        _settingsService.Load();
        var settings = _settingsService.Current;
        _currentModifiers = settings.HotkeyModifiers;
        _currentKey       = settings.HotkeyKey;

        // ۴. اعمال تم
        ThemeManager.ApplyTheme(settings.IsDarkTheme);

        // ۵. ایجاد پنجره‌ی مخفی برای دریافت پیام WM_HOTKEY
        _hiddenHost = new HiddenHostWindow();
        _hiddenHost.EnsureHandleCreated();

        // ۶. ثبت میانبر سراسری
        var hotkeyService = new HotkeyService();
        hotkeyService.Initialize(_hiddenHost.Handle);
        _hotkeyService = hotkeyService;

        bool registered = _hotkeyService.RegisterHotkey(_currentModifiers, _currentKey);
        if (!registered)
        {
            MessageBox.Show(
                $"میانبر {_currentKey} در حال حاضر توسط برنامه‌ی دیگری اشغال است.\n" +
                "از تنظیمات برنامه یک میانبر دیگر انتخاب کنید.",
                "Langwich – خطا در ثبت میانبر",
                MessageBoxButton.OK,
                MessageBoxImage.Warning);
        }

        _hotkeyService.HotkeyPressed += OnHotkeyPressed;

        // ۷. نمایش آیکون سینی
        _trayIconService = new TrayIconService();
        _trayIconService.SettingsRequested += OnSettingsRequested;
        _trayIconService.ExitRequested     += OnExitRequested;
        _trayIconService.Initialize();
    }

    // ---- هندلر میانبر ----

    private async void OnHotkeyPressed(object? sender, EventArgs e)
    {
        try
        {
            // ۱. دریافت متن انتخاب‌شده
            var selectedText = await _textSelectionService!.GetSelectedTextAsync();

            if (string.IsNullOrEmpty(selectedText))
            {
                return;
            }

            // ۲. تبدیل
            var result = _converterService!.Convert(selectedText);

            if (!result.WasConverted)
            {
                return;
            }

            // ۳. جایگزینی متن در برنامه‌ی هدف
            await _textSelectionService.ReplaceSelectedTextAsync(result.ConvertedText);

            // ۴. تغییر زبان کیبورد مطابق با متن تبدیل‌شده
            SwitchKeyboardLayout(result.Direction);
        }
        catch (Exception)
        {
        }
    }

    // ---- هندلر تنظیمات ----

    private void OnSettingsRequested(object? sender, EventArgs e)
    {
        // اگر پنجره‌ی تنظیمات قبلاً باز است، آن را به جلو می‌آوریم
        foreach (Window w in Windows)
        {
            if (w is SettingsWindow existing)
            {
                existing.Activate();
                return;
            }
        }

        var vm = new SettingsViewModel(_settingsService!, _startupService!, _hotkeyService!);
        vm.ThemeChanged += (_, isDark) => ThemeManager.ApplyTheme(isDark);

        var win = new SettingsWindow { DataContext = vm };
        win.Show();
    }

    // ---- هندلر خروج ----

    private void OnExitRequested(object? sender, EventArgs e)
    {
        Shutdown();
    }

    protected override void OnExit(ExitEventArgs e)
    {
        _hotkeyService?.Dispose();
        _trayIconService?.Dispose();
        _hiddenHost?.Close();

        if (_mutex is not null)
        {
            try { _mutex.ReleaseMutex(); } catch { }
            _mutex.Dispose();
        }

        base.OnExit(e);
    }

    // ---- تغییر زبان کیبورد ----

    private static void SwitchKeyboardLayout(Models.ConversionDirection direction)
    {
        try
        {
            string klid = direction == Models.ConversionDirection.ToPersian
                ? Helpers.NativeMethods.KLID_PERSIAN
                : Helpers.NativeMethods.KLID_ENGLISH;

            var hkl = Helpers.NativeMethods.LoadKeyboardLayout(klid, Helpers.NativeMethods.KLF_ACTIVATE);
            if (hkl != IntPtr.Zero)
            {
                Helpers.NativeMethods.ActivateKeyboardLayout(hkl, Helpers.NativeMethods.KLF_NOTELSGLOBAL);
            }
        }
        catch
        {
            // اگه تغییر زبان ممکن نبود، نادیده بگیر
        }
    }
}
