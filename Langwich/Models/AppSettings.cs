using System.Text.Json.Serialization;
using System.Windows.Input;

namespace Langwich.Models;

/// <summary>
/// تنظیمات قابل ذخیره‌ی برنامه. در %AppData%\Langwich\settings.json ذخیره می‌شود.
/// </summary>
public sealed class AppSettings
{
    [JsonConverter(typeof(JsonStringEnumConverter))]
    public Key HotkeyKey { get; set; } = Key.F10;

    [JsonConverter(typeof(JsonStringEnumConverter))]
    public ModifierKeys HotkeyModifiers { get; set; } = ModifierKeys.None;

    public bool StartWithWindows { get; set; } = false;
    public bool NotificationsEnabled { get; set; } = true;
    public bool IsDarkTheme { get; set; } = true;

    /// <summary>
    /// اگر true باشد، حرف «پ» در تبدیل فارسی→انگلیسی به کلید \ نگاشت می‌شود
    /// (چیدمان Persian Standard). false = کلید m (چیدمان Microsoft Persian پیش‌فرض).
    /// </summary>
    public bool UseAlternatePeKey { get; set; } = false;

    /// <summary>
    /// اگر true باشد، بعد از تبدیل متن، زبان کیبورد ویندوز هم عوض می‌شود
    /// (فارسی برای متن فارسی، انگلیسی برای متن انگلیسی).
    /// </summary>
    public bool SwitchKeyboardLayoutAfterConvert { get; set; } = true;
}
