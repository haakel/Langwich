namespace Langwich.Services.Interfaces;

/// <summary>
/// سرویس مسئول دریافت متن انتخاب‌شده از هر برنامه‌ای و جایگزینی آن با متن تبدیل‌شده.
/// روش کار: شبیه‌سازی Ctrl+C برای کپی و Ctrl+V برای جایگزینی از طریق کلیپ‌بورد.
/// </summary>
public interface ITextSelectionService
{
    /// <summary>
    /// متن انتخاب‌شده‌ی فعلی در هر برنامه‌ای را برمی‌گرداند.
    /// اگر هیچ متنی انتخاب نشده باشد یا عملیات ناموفق باشد، null برمی‌گرداند.
    /// </summary>
    Task<string?> GetSelectedTextAsync();

    /// <summary>
    /// متن انتخاب‌شده‌ی فعلی را با متن جدید جایگزین می‌کند (شبیه‌سازی Ctrl+V).
    /// </summary>
    Task ReplaceSelectedTextAsync(string newText);
}
