namespace Langwich.Services.Interfaces;

/// <summary>
/// سرویس مسئول تنظیم و حذف اجرای خودکار برنامه در هنگام ورود به ویندوز
/// از طریق رجیستری HKCU (بدون نیاز به دسترسی Administrator).
/// </summary>
public interface IStartupService
{
    /// <summary>آیا Langwich در حال حاضر در لیست اجرای خودکار ویندوز هست یا نه.</summary>
    bool IsStartupEnabled { get; }

    /// <summary>وضعیت اجرای خودکار را فعال یا غیرفعال می‌کند.</summary>
    void SetStartup(bool enable);

    /// <summary>آیا میانبر (.lnk) روی دسکتاپ وجود دارد؟</summary>
    bool IsDesktopShortcutCreated { get; }

    /// <summary>میانبر دسکتاپ را می‌سازد یا حذف می‌کند.</summary>
    void SetDesktopShortcut(bool create);
}
