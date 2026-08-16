using System.Windows;
using System.Windows.Input;
using Langwich.Helpers;
using Langwich.ViewModels;

namespace Langwich.Views;

/// <summary>
/// Code-behind پنجره‌ی تنظیمات. تنها وظیفه‌اش این است که رویداد KeyDown را
/// به ViewModel منتقل کند تا میانبر ضبط شود.
/// </summary>
public partial class SettingsWindow : Window
{
    public SettingsWindow()
    {
        InitializeComponent();
        DataContextChanged += OnDataContextChanged;
    }

    private void OnDataContextChanged(object sender, DependencyPropertyChangedEventArgs e)
    {
        if (e.OldValue is SettingsViewModel oldVm)
            oldVm.RequestClose -= OnRequestClose;

        if (e.NewValue is SettingsViewModel newVm)
            newVm.RequestClose += OnRequestClose;
    }

    private void OnRequestClose(object? sender, EventArgs e) => Close();

    private void Window_PreviewKeyDown(object sender, KeyEventArgs e)
    {
        if (DataContext is SettingsViewModel vm && vm.IsListeningForHotkey)
        {
            // کلید واقعی را از System.Windows.Input.Key دریافت می‌کنیم
            var key = e.Key == Key.System ? e.SystemKey : e.Key;
            vm.CaptureHotkey(Keyboard.Modifiers, key);
            e.Handled = true;
        }
    }

    protected override void OnClosed(EventArgs e)
    {
        base.OnClosed(e);
        // اتصال رویداد را قطع می‌کنیم تا memory leak نداشته باشیم
        if (DataContext is SettingsViewModel vm)
            vm.RequestClose -= OnRequestClose;
    }
}
