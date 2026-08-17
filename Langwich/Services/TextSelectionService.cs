using System.Runtime.InteropServices;
using System.Windows;
using System.Windows.Interop;
using Langwich.Helpers;
using Langwich.Services.Interfaces;

namespace Langwich.Services;

/// <summary>
/// متن انتخاب‌شده را از هر برنامه‌ای می‌خواند و جایگزین می‌کند.
/// روش: پشتیبان‌گیری از کلیپ‌بورد → شبیه‌سازی Ctrl+C → خواندن کلیپ‌بورد → بازیابی پشتیبان.
/// </summary>
public sealed class TextSelectionService : ITextSelectionService
{
    private const int ClipboardPollTimeoutMs = 500;
    private const int ClipboardPollIntervalMs = 30;
    private static readonly SemaphoreSlim _lock = new(1, 1);

    // نام کلاس پنجره‌های Langwich برای جلوگیری از ارسال کلید به خودمان
    private const string LangwichWindowClassName = "HwndWrapper[Langwich;;]";

    public async Task<string?> GetSelectedTextAsync()
    {
        if (!await _lock.WaitAsync(0)) return null;
        try { return await GetSelectedTextCoreAsync(); }
        finally { _lock.Release(); }
    }

    private async Task<string?> GetSelectedTextCoreAsync()
    {
        // اگر پنجره خود Langwich فوکوس باشد، کاری نکن
        if (IsLangwichFocused()) return null;

        // ۱. پشتیبان‌گیری + شماره ترتیبی کلیپ‌بورد
        string? backup = null;
        uint seqBefore = 0;
        try
        {
            backup = GetClipboardTextSafe();
            Application.Current.Dispatcher.Invoke(() =>
            {
                seqBefore = NativeMethods.GetClipboardSequenceNumber();
                try { Clipboard.Clear(); } catch { }
            });
        }
        catch { }

        // ۲. شبیه‌سازی Ctrl+C
        KeybdService.SimulateCtrlKey(KeybdService.VK_C);

        // ۳. انتظار برای پر شدن کلیپ‌بورد
        string? result = await WaitForClipboardTextAsync();

        // ۴. بازیابی کلیپ‌بورد فقط اگر کسی تغییرش نداده باشد
        _ = Task.Run(async () =>
        {
            try
            {
                await Task.Delay(300);
                uint seqAfter = 0;
                Application.Current?.Dispatcher.Invoke(() =>
                    seqAfter = NativeMethods.GetClipboardSequenceNumber());

                // فقط بازیابی کن اگر کلیپ‌بورد توسط برنامه دیگری تغییر نکرده
                if (seqAfter == seqBefore || seqAfter == 0)
                    RestoreClipboard(backup);
            }
            catch { }
        });

        return result;
    }

    public async Task ReplaceSelectedTextAsync(string newText)
    {
        if (!await _lock.WaitAsync(0)) return;
        try
        {
            if (IsLangwichFocused()) return;

            string? backup = null;
            try { backup = GetClipboardTextSafe(); } catch { }

            SetClipboardTextSafe(newText);
            await Task.Delay(100);

            KeybdService.SimulateCtrlKey(KeybdService.VK_V);

            _ = Task.Run(async () =>
            {
                try
                {
                    await Task.Delay(350);
                    RestoreClipboard(backup);
                }
                catch { }
            });
        }
        finally { _lock.Release(); }
    }

    // --- چک کردن فوکوس پنجره ---

    private static bool IsLangwichFocused()
    {
        try
        {
            IntPtr foregroundHwnd = NativeMethods.GetForegroundWindow();
            if (foregroundHwnd == IntPtr.Zero) return false;

            var sb = new System.Text.StringBuilder(256);
            NativeMethods.GetWindowClassName(foregroundHwnd, sb, sb.Capacity);
            string className = sb.ToString();

            // اگر پنجره Langwich باشد، کلید را ارسال نکن
            return className.Contains("HwndWrapper") && className.Contains("Langwich");
        }
        catch { return false; }
    }

    // --- متدهای کمکی کلیپ‌بورد ---

    private static async Task<string?> WaitForClipboardTextAsync()
    {
        int elapsed = 0;
        while (elapsed < ClipboardPollTimeoutMs)
        {
            await Task.Delay(ClipboardPollIntervalMs);
            elapsed += ClipboardPollIntervalMs;
            var text = GetClipboardTextSafe();
            if (!string.IsNullOrEmpty(text)) return text;
        }
        return null;
    }

    private static string? GetClipboardTextSafe()
    {
        string? result = null;
        Application.Current?.Dispatcher.Invoke(() =>
        {
            try { result = Clipboard.ContainsText() ? Clipboard.GetText() : null; }
            catch { }
        });
        return result;
    }

    private static void SetClipboardTextSafe(string text)
    {
        Application.Current?.Dispatcher.Invoke(() =>
        {
            try { Clipboard.SetText(text); }
            catch { }
        });
    }

    private static void RestoreClipboard(string? text)
    {
        Application.Current?.Dispatcher.Invoke(() =>
        {
            try
            {
                if (string.IsNullOrEmpty(text)) Clipboard.Clear();
                else Clipboard.SetText(text);
            }
            catch { }
        });
    }
}
