namespace Langwich.ViewModels;

/// <summary>
/// ViewModel پنجره‌ی اعلان موقت. فقط پیام نمایشی را نگه می‌دارد.
/// </summary>
public sealed class NotificationViewModel : ViewModelBase
{
    private string _message = string.Empty;

    public string Message
    {
        get => _message;
        set => SetProperty(ref _message, value);
    }

    public NotificationViewModel(string message)
    {
        _message = message;
    }
}
