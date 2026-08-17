using System.Windows.Input;

namespace Langwich.Services.Interfaces;

/// <summary>
/// سرویس ثبت و مدیریت میانبر سراسری کیبورد.
/// </summary>
public interface IHotkeyService : IDisposable
{
    /// <summary>وقتی میانبر ثبت‌شده فشرده شود رخ می‌دهد.</summary>
    event EventHandler? HotkeyPressed;

    /// <summary>
    /// سرویس را با Handle پنجره‌ی میزبان راه‌اندازی می‌کند.
    /// باید قبل از RegisterHotkey فراخوانی شود.
    /// </summary>
    void Initialize(IntPtr windowHandle);

    /// <summary>یک میانبر سراسری جدید ثبت می‌کند. اگر ثبت موفق باشد true برمی‌گرداند.</summary>
    bool RegisterHotkey(ModifierKeys modifiers, Key key);

    /// <summary>میانبر ثبت‌شده‌ی فعلی را حذف می‌کند.</summary>
    void UnregisterHotkey();
}
