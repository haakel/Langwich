using System.Runtime.InteropServices;

namespace Langwich.Helpers;

internal static class NativeMethods
{
    [DllImport("user32.dll", SetLastError = true)]
    internal static extern bool RegisterHotKey(IntPtr hWnd, int id, uint fsModifiers, uint vk);

    [DllImport("user32.dll", SetLastError = true)]
    internal static extern bool UnregisterHotKey(IntPtr hWnd, int id);

    internal const int WM_HOTKEY = 0x0312;
    internal const uint MOD_NONE    = 0x0000;
    internal const uint MOD_ALT     = 0x0001;
    internal const uint MOD_CONTROL = 0x0002;
    internal const uint MOD_SHIFT   = 0x0004;
    internal const uint MOD_WIN     = 0x0008;

    [StructLayout(LayoutKind.Sequential)]
    internal struct POINT { public int X; public int Y; }

    [DllImport("user32.dll")]
    internal static extern bool GetCursorPos(out POINT lpPoint);

    internal const uint INPUT_KEYBOARD    = 1u;
    internal const uint KEYEVENTF_KEYUP   = 0x0002;
    [DllImport("user32.dll")]
    internal static extern uint GetClipboardSequenceNumber();

    [DllImport("user32.dll")]
    internal static extern IntPtr GetForegroundWindow();

    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    internal static extern int GetWindowClassName(IntPtr hWnd, System.Text.StringBuilder lpClassName, int nMaxCount);

    // --- تغییر زبان کیبورد ---
    [DllImport("user32.dll")]
    internal static extern IntPtr LoadKeyboardLayout(string pwszKLID, uint Flags);

    [DllImport("user32.dll")]
    internal static extern IntPtr ActivateKeyboardLayout(IntPtr hkl, uint Flags);

    [DllImport("user32.dll")]
    internal static extern IntPtr GetKeyboardLayout(uint idThread);

    [DllImport("user32.dll")]
    internal static extern uint GetWindowThreadProcessId(IntPtr hWnd, out uint lpdwProcessId);

    [DllImport("user32.dll")]
    internal static extern bool PostMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);

    internal const uint WM_INPUTLANGCHANGEREQUEST = 0x0050;
    internal const uint INPUTLANGCHANGE_FORWARD   = 0x0002;

    internal const uint KLF_ACTIVATE = 0x00000001;
    internal const uint KLF_NOTELSGLOBAL = 0x00000004;

    // شناسه‌های زبان کیبورد
    internal const string KLID_ENGLISH = "00000409";  // English (US)
    internal const string KLID_PERSIAN = "00000429";  // Persian (Farsi)
}
