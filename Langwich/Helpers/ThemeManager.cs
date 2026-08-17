using System.Windows;

namespace Langwich.Helpers;

public static class ThemeManager
{
    private const string DarkThemeUri  = "Resources/Themes/DarkTheme.xaml";
    private const string LightThemeUri = "Resources/Themes/LightTheme.xaml";

    public static void ApplyTheme(bool isDark)
    {
        var app = Application.Current;
        if (app is null) return;

        var newDictionary = new ResourceDictionary
        {
            Source = new Uri(isDark ? DarkThemeUri : LightThemeUri, UriKind.Relative)
        };

        var merged = app.Resources.MergedDictionaries;
        if (merged.Count > 0)
            merged[0] = newDictionary;
        else
            merged.Add(newDictionary);
    }
}
