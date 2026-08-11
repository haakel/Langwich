using System.Windows;
using System.Windows.Threading;
using DevOver.Helpers;
using DevOver.Services.Interfaces;
using DevOver.ViewModels;
using DevOver.Views;

namespace DevOver.Services;

/// <summary>
/// یک پنجره‌ی اعلان کوچک نزدیک موقعیت فعلی موس نمایش می‌دهد.
/// پنجره بعد از ۱٫۵ ثانیه به‌صورت Fade-out محو می‌شود.
/// </summary>
public sealed class NotificationService : INotificationService
{
    public void Show(string message)
    {
        // باید در UI Thread اجرا شود
        Application.Current?.Dispatcher.Invoke(() =>
        {
            NativeMethods.GetCursorPos(out var cursorPos);

            var popup = new NotificationPopup
            {
                DataContext = new NotificationViewModel(message)
            };

            // نمایش کمی بالاتر از موس
            popup.Left = cursorPos.X + 10;
            popup.Top  = cursorPos.Y - 60;

            popup.Show();

            // بعد از ۱٫۵ ثانیه، Fade-out و بسته می‌شود
            var timer = new DispatcherTimer
            {
                Interval = TimeSpan.FromMilliseconds(1500)
            };
            timer.Tick += (_, _) =>
            {
                timer.Stop();
                popup.FadeOutAndClose();
            };
            timer.Start();
        });
    }
}
