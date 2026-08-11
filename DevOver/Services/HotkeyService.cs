using System.Windows;
using System.Windows.Input;
using System.Windows.Interop;
using DevOver.Helpers;
using DevOver.Services.Interfaces;

namespace DevOver.Services;

/// <summary>
/// میانبر سراسری کیبورد را از طریق Win32 RegisterHotKey ثبت می‌کند.
/// برای دریافت پیام WM_HOTKEY از HwndSource پنجره‌ی مخفی برنامه استفاده می‌کند.
/// </summary>
public sealed class HotkeyService : IHotkeyService
{
    private const int HotkeyId = 9001;

    private HwndSource? _hwndSource;
    private bool _isRegistered;

    public event EventHandler? HotkeyPressed;

    /// <summary>
    /// سرویس را با دسته‌ی (handle) پنجره‌ی مخفی راه‌اندازی می‌کند.
    /// باید قبل از RegisterHotkey فراخوانی شود.
    /// </summary>
    public void Initialize(IntPtr windowHandle)
    {
        _hwndSource = HwndSource.FromHwnd(windowHandle);
        _hwndSource?.AddHook(WndProc);
    }

    public bool RegisterHotkey(ModifierKeys modifiers, Key key)
    {
        UnregisterHotkey();

        if (_hwndSource is null)
        {
            return false;
        }

        uint winMod = ConvertModifiers(modifiers);
        uint vk = (uint)KeyInterop.VirtualKeyFromKey(key);

        _isRegistered = NativeMethods.RegisterHotKey(_hwndSource.Handle, HotkeyId, winMod, vk);
        return _isRegistered;
    }

    public void UnregisterHotkey()
    {
        if (_isRegistered && _hwndSource is not null)
        {
            NativeMethods.UnregisterHotKey(_hwndSource.Handle, HotkeyId);
            _isRegistered = false;
        }
    }

    private IntPtr WndProc(IntPtr hwnd, int msg, IntPtr wParam, IntPtr lParam, ref bool handled)
    {
        if (msg == NativeMethods.WM_HOTKEY && wParam.ToInt32() == HotkeyId)
        {
            HotkeyPressed?.Invoke(this, EventArgs.Empty);
            handled = true;
        }
        return IntPtr.Zero;
    }

    /// <summary>ModifierKeys ویندوز پرزنتیشن فانداشن را به مقادیر Win32 تبدیل می‌کند.</summary>
    private static uint ConvertModifiers(ModifierKeys modifiers)
    {
        uint result = NativeMethods.MOD_NONE;
        if (modifiers.HasFlag(ModifierKeys.Alt))     result |= NativeMethods.MOD_ALT;
        if (modifiers.HasFlag(ModifierKeys.Control)) result |= NativeMethods.MOD_CONTROL;
        if (modifiers.HasFlag(ModifierKeys.Shift))   result |= NativeMethods.MOD_SHIFT;
        if (modifiers.HasFlag(ModifierKeys.Windows)) result |= NativeMethods.MOD_WIN;
        return result;
    }

    public void Dispose()
    {
        UnregisterHotkey();
        _hwndSource?.RemoveHook(WndProc);
        _hwndSource?.Dispose();
    }
}
