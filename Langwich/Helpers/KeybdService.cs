using System.Runtime.InteropServices;

namespace Langwich.Helpers;

/// <summary>
/// شبیه‌سازی کلید با keybd_event (API ساده و قابل اعتماد ویندوز).
/// </summary>
internal static class KeybdService
{
    internal const byte VK_CONTROL = 0x11;
    internal const byte VK_C = 0x43;
    internal const byte VK_V = 0x56;

    [DllImport("user32.dll")]
    private static extern void keybd_event(byte bVk, byte bScan, uint dwFlags, UIntPtr dwExtraInfo);

    private const uint KEYEVENTF_KEYUP = 0x0002;

    internal static void KeyDown(byte vk) => keybd_event(vk, 0, 0, UIntPtr.Zero);
    internal static void KeyUp(byte vk) => keybd_event(vk, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);

    internal static void SimulateCtrlKey(byte keyVk)
    {
        KeyDown(VK_CONTROL);
        KeyDown(keyVk);
        KeyUp(keyVk);
        KeyUp(VK_CONTROL);
    }
}
