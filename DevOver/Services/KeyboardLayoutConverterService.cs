using DevOver.Models;
using DevOver.Services.Interfaces;

namespace DevOver.Services;

/// <summary>
/// منطق اصلی تبدیل چیدمان کیبورد. جدول نگاشت بر اساس چیدمان استاندارد
/// کیبورد فارسی ایرانی (Microsoft Persian) است.
/// مثال تأیید‌شده توسط کاربر: "sghl" (انگلیسی با چیدمان فارسی) → "سلام"
/// </summary>
public sealed class KeyboardLayoutConverterService : IKeyboardLayoutConverterService
{
    // نگاشت کلیدهای انگلیسی به حروف فارسی (چیدمان استاندارد کیبورد ایرانی Microsoft)
    // ردیف Q: q=ض  w=ص  e=ث  r=ق  t=ف  y=غ  u=ع  i=ه  o=خ  p=ح  [=ج  ]=چ
    // ردیف A: a=ش  s=س  d=ی  f=ب  g=ل  h=ا  j=ت  k=ن  l=م  ;=ک  '=گ
    // ردیف Z: z=ظ  x=ط  c=ز  v=ر  b=ذ  n=د  m=پ  ,=و  .=.  /=/
    private static readonly Dictionary<char, char> EnglishToPersian = new()
    {
        // ردیف بالا
        ['`'] = '٭', ['1'] = '۱', ['2'] = '۲', ['3'] = '۳', ['4'] = '۴',
        ['5'] = '۵', ['6'] = '۶', ['7'] = '۷', ['8'] = '۸', ['9'] = '۹', ['0'] = '۰',
        ['q'] = 'ض', ['w'] = 'ص', ['e'] = 'ث', ['r'] = 'ق',
        ['t'] = 'ف', ['y'] = 'غ', ['u'] = 'ع', ['i'] = 'ه',
        ['o'] = 'خ', ['p'] = 'ح', ['['] = 'ج', [']'] = 'چ',
        // ردیف وسط
        ['a'] = 'ش', ['s'] = 'س', ['d'] = 'ی', ['f'] = 'ب',
        ['g'] = 'ل', ['h'] = 'ا', ['j'] = 'ت', ['k'] = 'ن',
        ['l'] = 'م', [';'] = 'ک', ['\''] = 'گ',
        // ردیف پایین
        ['z'] = 'ظ', ['x'] = 'ط', ['c'] = 'ز', ['v'] = 'ر',
        ['b'] = 'ذ', ['n'] = 'د', ['m'] = 'پ', [','] = 'و', ['/'] = '؟',
    };

    // نگاشت معکوس (فارسی → انگلیسی)
    private static readonly Dictionary<char, char> PersianToEnglish =
        EnglishToPersian.ToDictionary(kv => kv.Value, kv => kv.Key);

    public ConversionResult Convert(string text)
    {
        if (string.IsNullOrEmpty(text))
        {
            return new ConversionResult { OriginalText = text, ConvertedText = text, Direction = ConversionDirection.None };
        }

        // نرمال‌سازی یونیکد: حروف عربی → فارسی
        var normalized = text
            .Replace('\u0643', 'ک')  // ك عربی → ک فارسی
            .Replace('\u064A', 'ی')  // ي عربی → ی فارسی
            .Replace('\u0629', 'ه')  // ة (تاء مربوطه) → ه
            .Replace('\u0621', 'ء')  // ء (همزه)
            .Replace('\u0622', 'آ')  // آ
            .Replace('\u0623', 'ا')  // أ → ا
            .Replace('\u0625', 'ا')  // إ → ا
            .Replace('\u0624', 'و')  // ؤ → و
            .Replace('\u0626', 'ی')  // ئ → ی
            .Replace('\u0671', 'ا'); // ٱ → ا

        // شمارش حروف فارسی و حروف ASCII
        int persianCount = normalized.Count(IsPersianLetter);
        int latinCount = normalized.Count(c => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z'));

        ConversionDirection direction;
        string converted;

        if (persianCount > 0 && persianCount >= latinCount)
        {
            // فارسی غالب → به انگلیسی تبدیل کن
            direction = ConversionDirection.ToEnglish;
            converted = ConvertToEnglish(normalized);
        }
        else if (latinCount > 0 && latinCount > persianCount)
        {
            // انگلیسی غالب → به فارسی تبدیل کن
            direction = ConversionDirection.ToPersian;
            converted = ConvertToPersian(normalized);
        }
        else
        {
            direction = ConversionDirection.None;
            converted = text;
        }

        return new ConversionResult
        {
            OriginalText = text,
            ConvertedText = converted,
            Direction = direction
        };
    }

    public string ConvertToPersian(string englishText)
    {
        var sb = new System.Text.StringBuilder(englishText.Length);
        foreach (char c in englishText)
        {
            // حروف کوچک را مستقیم نگاشت می‌کنیم
            if (EnglishToPersian.TryGetValue(char.ToLower(c), out char persian))
            {
                sb.Append(persian);
            }
            else
            {
                // اعداد، فاصله و علائمی که نگاشت ندارند بدون تغییر می‌مانند
                sb.Append(c);
            }
        }
        return sb.ToString();
    }

    public string ConvertToEnglish(string persianText)
    {
        var sb = new System.Text.StringBuilder(persianText.Length);
        foreach (char c in persianText)
        {
            if (PersianToEnglish.TryGetValue(c, out char english))
            {
                sb.Append(english);
            }
            else
            {
                sb.Append(c);
            }
        }
        return sb.ToString();
    }

    /// <summary>بررسی می‌کند که آیا یک کاراکتر در محدوده‌ی یونیکد فارسی/عربی است.</summary>
    private static bool IsPersianLetter(char c) => c >= '\u0600' && c <= '\u06FF';
}
