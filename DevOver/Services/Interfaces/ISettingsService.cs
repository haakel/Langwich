using DevOver.Models;

namespace DevOver.Services.Interfaces;

/// <summary>
/// سرویس مسئول بارگذاری و ذخیره‌سازی تنظیمات برنامه در فایل JSON.
/// </summary>
public interface ISettingsService
{
    /// <summary>تنظیمات فعلی برنامه.</summary>
    AppSettings Current { get; }

    /// <summary>تنظیمات را از فایل JSON بارگذاری می‌کند. اگر فایل وجود نداشت، مقادیر پیش‌فرض برمی‌گردد.</summary>
    void Load();

    /// <summary>تنظیمات فعلی را در فایل JSON ذخیره می‌کند.</summary>
    void Save();
}
