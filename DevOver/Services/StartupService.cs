using Microsoft.Win32;
using DevOver.Services.Interfaces;

namespace DevOver.Services;

/// <summary>
/// اجرای خودکار DevOver در هنگام ورود به ویندوز را از طریق رجیستری
/// HKCU\Software\Microsoft\Windows\CurrentVersion\Run مدیریت می‌کند.
/// هیچ دسترسی Administrator ای نیاز ندارد.
/// </summary>
public sealed class StartupService : IStartupService
{
    private const string RegistryKeyPath = @"Software\Microsoft\Windows\CurrentVersion\Run";
    private const string ValueName = "Langwich";

    public bool IsStartupEnabled
    {
        get
        {
            try
            {
                using var key = Registry.CurrentUser.OpenSubKey(RegistryKeyPath, writable: false);
                return key?.GetValue(ValueName) is not null;
            }
            catch
            {
                return false;
            }
        }
    }

    public void SetStartup(bool enable)
    {
        try
        {
            using var key = Registry.CurrentUser.OpenSubKey(RegistryKeyPath, writable: true);
            if (key is null) return;

            if (enable)
            {
                var exePath = Environment.ProcessPath ?? string.Empty;
                if (!string.IsNullOrEmpty(exePath))
                {
                    key.SetValue(ValueName, exePath);
                }
            }
            else
            {
                key.DeleteValue(ValueName, throwOnMissingValue: false);
            }
        }
        catch
        {
            // خطای رجیستری را نادیده می‌گیریم
        }
    }
}
