using System.Text.Json.Serialization;
using System.Windows.Input;

namespace DevOver.Models;

/// <summary>
/// تنظیمات قابل ذخیره‌ی برنامه. در %AppData%\DevOver\settings.json ذخیره می‌شود.
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
}
