using Langwich.Models;

namespace Langwich.Services.Interfaces;

/// <summary>
/// سرویس تبدیل چیدمان کیبورد بین انگلیسی و فارسی.
/// </summary>
public interface IKeyboardLayoutConverterService
{
    /// <summary>
    /// متن ورودی را بررسی و به‌صورت خودکار تبدیل می‌کند.
    /// اگر متن فارسی غالب باشد به انگلیسی و بالعکس.
    /// </summary>
    ConversionResult Convert(string text);

    /// <summary>متن انگلیسی (با چیدمان فارسی) را به فارسی تبدیل می‌کند.</summary>
    string ConvertToPersian(string englishText);

    /// <summary>متن فارسی را به انگلیسی (با چیدمان فارسی) تبدیل می‌کند.</summary>
    string ConvertToEnglish(string persianText);

    /// <summary>
    /// اگر true باشد، حرف «پ» در تبدیل فارسی←انگلیسی به کلید \ نگاشت می‌شود
    /// (چیدمان Persian Standard — کیبوردهایی که «پ» روی کلید \ دارند).
    /// پیش‌فرض false: کلید m (چیدمان Microsoft Persian).
    /// </summary>
    bool UseAlternatePeKey { get; set; }
}
