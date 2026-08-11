using System.Drawing;
using System.IO;
using System.Windows.Forms;
using DevOver.Services.Interfaces;

namespace DevOver.Services;

/// <summary>
/// آیکون برنامه را در سینی سیستم (System Tray) نمایش می‌دهد.
/// از System.Windows.Forms.NotifyIcon استفاده می‌کند (بدون نیاز به NuGet خارجی).
/// </summary>
public sealed class TrayIconService : ITrayIconService
{
    private NotifyIcon? _notifyIcon;

    public event EventHandler? SettingsRequested;
    public event EventHandler? ExitRequested;

    public void Initialize()
    {
        _notifyIcon = new NotifyIcon
        {
            Text    = "Langwich – تبدیل چیدمان کیبورد",
            Visible = true,
            Icon    = LoadIcon()
        };

        // منوی راست‌کلیک (RTL)
        var menu = new ContextMenuStrip
        {
            RightToLeft = RightToLeft.Yes
        };

        var settingsItem = new ToolStripMenuItem("⚙  تنظیمات");
        settingsItem.Click += (_, _) => SettingsRequested?.Invoke(this, EventArgs.Empty);

        var separator = new ToolStripSeparator();

        var exitItem = new ToolStripMenuItem("✕  خروج");
        exitItem.Click += (_, _) => ExitRequested?.Invoke(this, EventArgs.Empty);

        menu.Items.Add(settingsItem);
        menu.Items.Add(separator);
        menu.Items.Add(exitItem);

        _notifyIcon.ContextMenuStrip = menu;

        // دوبار کلیک روی آیکون هم پنجره‌ی تنظیمات را باز می‌کند
        _notifyIcon.DoubleClick += (_, _) => SettingsRequested?.Invoke(this, EventArgs.Empty);
    }

    private static Icon LoadIcon()
    {
        try
        {
            var iconPath = Path.Combine(AppContext.BaseDirectory, "Resources", "Icons", "langwich.ico");
            if (File.Exists(iconPath))
                return new Icon(iconPath);
        }
        catch { }

        // اگر فایل آیکون پیدا نشد، از آیکون پیش‌فرض سیستم استفاده می‌کنیم
        return SystemIcons.Application;
    }

    public void Dispose()
    {
        if (_notifyIcon is not null)
        {
            _notifyIcon.Visible = false;
            _notifyIcon.Dispose();
            _notifyIcon = null;
        }
    }
}
