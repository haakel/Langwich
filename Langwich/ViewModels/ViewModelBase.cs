using System.ComponentModel;
using System.Runtime.CompilerServices;

namespace Langwich.ViewModels;

/// <summary>
/// کلاس پایه برای تمام ViewModel ها. پیاده‌سازی INotifyPropertyChanged را فراهم می‌کند.
/// </summary>
public abstract class ViewModelBase : INotifyPropertyChanged
{
    public event PropertyChangedEventHandler? PropertyChanged;

    /// <summary>
    /// مقدار یک property را تغییر می‌دهد و در صورت تغییر واقعی، رویداد PropertyChanged را اعلام می‌کند.
    /// </summary>
    protected bool SetProperty<T>(ref T field, T value, [CallerMemberName] string? propertyName = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value))
            return false;

        field = value;
        OnPropertyChanged(propertyName);
        return true;
    }

    protected void OnPropertyChanged([CallerMemberName] string? propertyName = null)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }
}
