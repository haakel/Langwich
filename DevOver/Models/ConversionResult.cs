namespace DevOver.Models;

/// <summary>
/// نتیجه‌ی یک عملیات تبدیل چیدمان کیبورد را نگه می‌دارد.
/// </summary>
public sealed class ConversionResult
{
    public string OriginalText { get; init; } = string.Empty;
    public string ConvertedText { get; init; } = string.Empty;
    public ConversionDirection Direction { get; init; } = ConversionDirection.None;
    public bool WasConverted => Direction != ConversionDirection.None;
}
