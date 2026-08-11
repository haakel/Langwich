namespace DevOver.Services.Interfaces;

/// <summary>
/// سرویس مسئول نمایش پنجره‌ی اعلان موقت (Popup) نزدیک موقعیت موس
/// بعد از هر عملیات تبدیل موفق.
/// </summary>
public interface INotificationService
{
    /// <summary>
    /// یک پنجره‌ی اعلان کوچک با پیام مشخص‌شده نمایش می‌دهد.
    /// پنجره بعد از مدت کوتاهی به‌صورت خودکار محو می‌شود.
    /// </summary>
    void Show(string message);
}
