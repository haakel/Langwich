namespace Langwich.Services.Interfaces;

/// <summary>
/// سرویس مسئول نمایش و مدیریت آیکون برنامه در سینی سیستم (System Tray).
/// </summary>
public interface ITrayIconService : IDisposable
{
    /// <summary>وقتی کاربر «تنظیمات» را از منوی سینی انتخاب کند رخ می‌دهد.</summary>
    event EventHandler? SettingsRequested;

    /// <summary>وقتی کاربر «خروج» را از منوی سینی انتخاب کند رخ می‌دهد.</summary>
    event EventHandler? ExitRequested;

    /// <summary>آیکون سینی را ایجاد و نمایش می‌دهد.</summary>
    void Initialize();
}
