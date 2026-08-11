using System.Windows.Input;
using DevOver.Helpers;
using DevOver.Models;
using DevOver.Services.Interfaces;

namespace DevOver.ViewModels;

/// <summary>
/// ViewModel پنجره‌ی تنظیمات. تمام منطق UI را از code-behind جدا می‌کند.
/// </summary>
public sealed class SettingsViewModel : ViewModelBase
{
    private readonly ISettingsService _settingsService;
    private readonly IStartupService  _startupService;
    private readonly IHotkeyService   _hotkeyService;

    // ---- Properties ----

    private ModifierKeys _hotkeyModifiers;
    public ModifierKeys HotkeyModifiers
    {
        get => _hotkeyModifiers;
        set => SetProperty(ref _hotkeyModifiers, value);
    }

    private Key _hotkeyKey;
    public Key HotkeyKey
    {
        get => _hotkeyKey;
        set => SetProperty(ref _hotkeyKey, value);
    }

    private bool _isListeningForHotkey;
    /// <summary>وقتی true است، دکمه‌ی «تغییر میانبر» در حالت ضبط کلید است.</summary>
    public bool IsListeningForHotkey
    {
        get => _isListeningForHotkey;
        set => SetProperty(ref _isListeningForHotkey, value);
    }

    private bool _startWithWindows;
    public bool StartWithWindows
    {
        get => _startWithWindows;
        set => SetProperty(ref _startWithWindows, value);
    }

    private bool _notificationsEnabled;
    public bool NotificationsEnabled
    {
        get => _notificationsEnabled;
        set => SetProperty(ref _notificationsEnabled, value);
    }

    private bool _isDarkTheme;
    public bool IsDarkTheme
    {
        get => _isDarkTheme;
        set => SetProperty(ref _isDarkTheme, value);
    }

    /// <summary>نمایش متنی میانبر فعلی برای دکمه‌ی تغییر میانبر.</summary>
    public string HotkeyDisplayText
    {
        get
        {
            if (IsListeningForHotkey) return "یک کلید بفشارید...";
            var mod = HotkeyModifiers == ModifierKeys.None
                ? ""
                : HotkeyModifiers.ToString().Replace(", ", "+") + "+";
            return mod + HotkeyKey.ToString();
        }
    }

    // ---- Events ----

    /// <summary>بعد از ذخیره‌ی تنظیمات ارسال می‌شود تا App.xaml.cs میانبر را دوباره ثبت کند.</summary>
    public event EventHandler<(ModifierKeys Modifiers, Key Key)>? HotkeyChanged;

    /// <summary>وقتی کاربر تم را تغییر دهد ارسال می‌شود تا ThemeManager در App.xaml.cs صدا زده شود.</summary>
    public event EventHandler<bool>? ThemeChanged;

    /// <summary>وقتی پنجره باید بسته شود ارسال می‌شود.</summary>
    public event EventHandler? RequestClose;

    // ---- Commands ----

    public ICommand StartListeningForHotkeyCommand { get; }
    public ICommand SaveCommand { get; }
    public ICommand CancelCommand { get; }

    // ---- Constructor ----

    public SettingsViewModel(
        ISettingsService settingsService,
        IStartupService startupService,
        IHotkeyService hotkeyService)
    {
        _settingsService = settingsService;
        _startupService  = startupService;
        _hotkeyService   = hotkeyService;

        // بارگذاری مقادیر فعلی از تنظیمات
        var s = _settingsService.Current;
        _hotkeyModifiers     = s.HotkeyModifiers;
        _hotkeyKey           = s.HotkeyKey;
        _startWithWindows    = s.StartWithWindows;
        _notificationsEnabled = s.NotificationsEnabled;
        _isDarkTheme         = s.IsDarkTheme;

        StartListeningForHotkeyCommand = new RelayCommand(() =>
        {
            IsListeningForHotkey = true;
            OnPropertyChanged(nameof(HotkeyDisplayText));
        });

        SaveCommand   = new RelayCommand(Save);
        CancelCommand = new RelayCommand(() => RequestClose?.Invoke(this, EventArgs.Empty));
    }

    // ---- Public Methods ----

    /// <summary>
    /// از Window_PreviewKeyDown صدا زده می‌شود تا کلید فشرده‌شده را به‌عنوان میانبر جدید ثبت کند.
    /// کلیدهای صرفاً Modifier (Ctrl، Alt، Shift، Win) نادیده گرفته می‌شوند.
    /// </summary>
    public void CaptureHotkey(ModifierKeys modifiers, Key key)
    {
        if (!IsListeningForHotkey) return;

        // کلیدهای Modifier تنها به‌عنوان میانبر قبول نمی‌شوند
        bool isPureModifier = key is Key.LeftCtrl or Key.RightCtrl
                                  or Key.LeftAlt  or Key.RightAlt
                                  or Key.LeftShift or Key.RightShift
                                  or Key.LWin or Key.RWin;
        if (isPureModifier) return;

        HotkeyModifiers      = modifiers;
        HotkeyKey            = key;
        IsListeningForHotkey = false;
        OnPropertyChanged(nameof(HotkeyDisplayText));
    }

    // ---- Private Methods ----

    private void Save()
    {
        var prevDark = _settingsService.Current.IsDarkTheme;

        _settingsService.Current.HotkeyModifiers     = HotkeyModifiers;
        _settingsService.Current.HotkeyKey           = HotkeyKey;
        _settingsService.Current.StartWithWindows    = StartWithWindows;
        _settingsService.Current.NotificationsEnabled = NotificationsEnabled;
        _settingsService.Current.IsDarkTheme         = IsDarkTheme;

        _settingsService.Save();

        // اعمال تنظیمات Startup در رجیستری
        _startupService.SetStartup(StartWithWindows);

        // اگر میانبر تغییر کرده، دوباره ثبت شود
        _hotkeyService.RegisterHotkey(HotkeyModifiers, HotkeyKey);
        HotkeyChanged?.Invoke(this, (HotkeyModifiers, HotkeyKey));

        // اگر تم تغییر کرده، ThemeManager را صدا بزن
        if (IsDarkTheme != prevDark)
        {
            ThemeChanged?.Invoke(this, IsDarkTheme);
        }

        RequestClose?.Invoke(this, EventArgs.Empty);
    }
}
