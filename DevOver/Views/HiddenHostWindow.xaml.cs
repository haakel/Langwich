using System.Windows;
using System.Windows.Interop;

namespace DevOver.Views;

/// <summary>
/// یک پنجره‌ی کاملاً نامرئی که فقط برای دریافت پیام Win32 WM_HOTKEY وجود دارد.
/// بدون این پنجره، HotkeyService نمی‌تواند میانبر سراسری را ثبت کند.
/// </summary>
public partial class HiddenHostWindow : Window
{
    public HiddenHostWindow()
    {
        InitializeComponent();
    }

    /// <summary>
    /// Handle پنجره را اجباراً ایجاد می‌کند (بدون نمایش پنجره).
    /// باید قبل از ارسال Handle به HotkeyService فراخوانی شود.
    /// </summary>
    public void EnsureHandleCreated()
    {
        var helper = new WindowInteropHelper(this);
        helper.EnsureHandle();
    }

    /// <summary>Handle (HWND) پنجره برای استفاده در RegisterHotKey.</summary>
    public IntPtr Handle => new WindowInteropHelper(this).Handle;
}
