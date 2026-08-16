using System.Windows;
using System.Windows.Media.Animation;
using System.Windows.Threading;

namespace Langwich.Views;

/// <summary>
/// پنجره‌ی خوش‌آمد (Splash) که هنگام شروع برنامه، لوگو را چند لحظه نمایش می‌دهد
/// تا کاربر بفهمد برنامه اجرا شده است.
/// </summary>
public partial class SplashWindow : Window
{
    public SplashWindow()
    {
        InitializeComponent();
    }

    /// <summary>
    /// با انیمیشن نوار پیشرفت نمایش داده می‌شود و بعد از مدت کوتاهی خودکار بسته می‌شود.
    /// </summary>
    public void ShowAndCloseAfter(TimeSpan duration)
    {
        Show();

        // انیمیشن نوار پیشرفت: از ۰ تا عرض کامل (عرض پنجره منهای حاشیه‌ها)
        double progressWidth = 380 - 100; // 380 عرض پنجره، 100 فاصله‌های کناری
        var anim = new DoubleAnimation(0, progressWidth, duration)
        {
            EasingFunction = new QuadraticEase { EasingMode = EasingMode.EaseInOut }
        };
        ProgressFill.BeginAnimation(WidthProperty, anim);

        // بستن خودکار بعد از مدت مشخص
        var timer = new DispatcherTimer { Interval = duration };
        timer.Tick += (_, _) =>
        {
            timer.Stop();
            Close();
        };
        timer.Start();
    }
}