using Microsoft.Win32;
using System.IO;
using Langwich.Services.Interfaces;

namespace Langwich.Services;

/// <summary>
/// اجرای خودکار Langwich در هنگام ورود به ویندوز را از طریق رجیستری
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

    // ================= شورت‌کات دسکتاپ =================

    /// <summary>
    /// آیا فایل میانبر (.lnk) روی دسکتاپ وجود دارد؟
    /// </summary>
    public bool IsDesktopShortcutCreated
    {
        get
        {
            try
            {
                var shortcutPath = GetDesktopShortcutPath();
                return File.Exists(shortcutPath);
            }
            catch
            {
                return false;
            }
        }
    }

    /// <summary>
    /// میانبر (.lnk) روی دسکتاپ می‌سازد یا حذف می‌کند.
    /// </summary>
    public void SetDesktopShortcut(bool create)
    {
        try
        {
            var shortcutPath = GetDesktopShortcutPath();

            if (!create)
            {
                if (File.Exists(shortcutPath))
                    File.Delete(shortcutPath);
                return;
            }

            var exePath = Environment.ProcessPath ?? string.Empty;
            if (string.IsNullOrEmpty(exePath)) return;

            // ساخت میانبر با WScript.Shell (بدون نیاز به COM Reference جداگانه)
            dynamic shell = Activator.CreateInstance(Type.GetTypeFromProgID("WScript.Shell"))!;
            dynamic shortcut = shell.CreateShortcut(shortcutPath);
            shortcut.TargetPath = exePath;
            shortcut.WorkingDirectory = Path.GetDirectoryName(exePath) ?? string.Empty;
            shortcut.Description = "Langwich – تبدیل چیدمان کیبورد";
            shortcut.Save();
            System.Runtime.InteropServices.Marshal.FinalReleaseComObject(shortcut);
            System.Runtime.InteropServices.Marshal.FinalReleaseComObject(shell);
        }
        catch
        {
            // اگر ساخت شورت‌کات ممکن نبود، نادیده بگیر
        }
    }

    private static string GetDesktopShortcutPath()
    {
        var desktop = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
        return Path.Combine(desktop, "Langwich.lnk");
    }
}
