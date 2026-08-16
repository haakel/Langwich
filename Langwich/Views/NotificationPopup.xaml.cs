using System.Windows;
using System.Windows.Media.Animation;

namespace Langwich.Views;

/// <summary>
/// پنجره‌ی اعلان موقت که با Fade-in ظاهر می‌شود و با Fade-out بسته می‌شود.
/// </summary>
public partial class NotificationPopup : Window
{
    public NotificationPopup()
    {
        InitializeComponent();
        Opacity = 0;
        Loaded += OnLoaded;
    }

    private void OnLoaded(object sender, RoutedEventArgs e)
    {
        // انیمیشن Fade-in هنگام باز شدن
        var fadeIn = new DoubleAnimation(0, 1, TimeSpan.FromMilliseconds(200))
        {
            EasingFunction = new CubicEase { EasingMode = EasingMode.EaseOut }
        };
        BeginAnimation(OpacityProperty, fadeIn);
    }

    /// <summary>
    /// انیمیشن Fade-out را اجرا می‌کند و بعد از اتمام، پنجره را می‌بندد.
    /// از NotificationService فراخوانی می‌شود.
    /// </summary>
    public void FadeOutAndClose()
    {
        var fadeOut = new DoubleAnimation(1, 0, TimeSpan.FromMilliseconds(300))
        {
            EasingFunction = new CubicEase { EasingMode = EasingMode.EaseIn }
        };
        fadeOut.Completed += (_, _) => Close();
        BeginAnimation(OpacityProperty, fadeOut);
    }
}
