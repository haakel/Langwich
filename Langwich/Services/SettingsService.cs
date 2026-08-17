using System.IO;
using System.Text.Json;
using System.Text.Json.Serialization;
using Langwich.Models;
using Langwich.Services.Interfaces;

namespace Langwich.Services;

/// <summary>
/// تنظیمات برنامه را در فایل JSON ذخیره و بارگذاری می‌کند.
/// مسیر فایل: %AppData%\Langwich\settings.json
/// </summary>
public sealed class SettingsService : ISettingsService
{
    private static readonly string SettingsDir =
        Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "Langwich");

    private static readonly string SettingsPath =
        Path.Combine(SettingsDir, "settings.json");

    private static readonly JsonSerializerOptions JsonOptions = new()
    {
        WriteIndented = true,
        Converters = { new JsonStringEnumConverter() }
    };

    public AppSettings Current { get; private set; } = new();

    public void Load()
    {
        try
        {
            if (File.Exists(SettingsPath))
            {
                var json = File.ReadAllText(SettingsPath);
                Current = JsonSerializer.Deserialize<AppSettings>(json, JsonOptions) ?? new AppSettings();
            }
        }
        catch
        {
            // اگر فایل خراب بود یا خطایی رخ داد، از تنظیمات پیش‌فرض استفاده می‌کنیم
            Current = new AppSettings();
        }
    }

    public void Save()
    {
        try
        {
            Directory.CreateDirectory(SettingsDir);
            var json = JsonSerializer.Serialize(Current, JsonOptions);
            // ذخیره اتمیک: اول به فایل موقت، بعد انتقال
            var tempPath = SettingsPath + ".tmp";
            File.WriteAllText(tempPath, json);
            File.Move(tempPath, SettingsPath, overwrite: true);
        }
        catch
        {
            // خطای ذخیره‌سازی را نادیده می‌گیریم تا برنامه کرش نکند
        }
    }
}
