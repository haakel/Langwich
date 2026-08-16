using Langwich.Models;
using Langwich.Services.Interfaces;

namespace Langwich.Services;

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
        // نکته: در چیدمان «Persian Standard» حرف «پ» روی کلید \ است (نه m).
        // هر دو کلید پذیرفته می‌شوند و برای تبدیل معکوس بر اساس تنظیم کاربر انتخاب می‌شود.
        // نکته: کاراکترهای نگارشی (! ? . ، ؛ و ...) در جدول نیستند تا دستکاری نشوند.
        private static readonly Dictionary<char, char> EnglishToPersian = new()
        {
            // ردیف بالا (توجه: / تبدیل نمی‌شود چون کاربر می‌خواهد ! و ؟ ثابت بمانند)
            ['`'] = '٭', ['1'] = '۱', ['2'] = '۲', ['3'] = '۳', ['4'] = '۴',
            ['5'] = '۵', ['6'] = '۶', ['7'] = '۷', ['8'] = '۸', ['9'] = '۹', ['0'] = '۰',
            ['q'] = 'ض', ['w'] = 'ص', ['e'] = 'ث', ['r'] = 'ق',
            ['t'] = 'ف', ['y'] = 'غ', ['u'] = 'ع', ['i'] = 'ه',
            ['o'] = 'خ', ['p'] = 'ح', ['['] = 'ج', [']'] = 'چ',
            // ردیف وسط
            ['a'] = 'ش', ['s'] = 'س', ['d'] = 'ی', ['f'] = 'ب',
            ['g'] = 'ل', ['h'] = 'ا', ['j'] = 'ت', ['k'] = 'ن',
            ['l'] = 'م', [';'] = 'ک', ['\''] = 'گ',
            // ردیف پایین (توجه: ,  و  /  تبدیل نمی‌شوند — نگارشی‌اند)
            ['z'] = 'ظ', ['x'] = 'ط', ['c'] = 'ز', ['v'] = 'ر',
            ['b'] = 'ذ', ['n'] = 'د', ['m'] = 'پ', ['\\'] = 'پ', [','] = 'و',
        };

    // نگاشت معکوس (فارسی → انگلیسی). برای «پ» بسته به تنظیم کاربر،
    // کلید m یا \ انتخاب می‌شود (چیدمان استاندارد در برابر Persian Standard).
    private Dictionary<char, char> _persianToEnglish = BuildPersianToEnglish(alternatePeKey: false);

    /// <summary>
    /// اگر true باشد، حرف «پ» در تبدیل فارسی←انگلیسی به کلید \ نگاشت می‌شود
    /// (چیدمان Persian Standard). پیش‌فرض false است (کلید m / چیدمان Microsoft Persian).
    /// </summary>
    public bool UseAlternatePeKey
    {
        get => _useAlternatePeKey;
        set
        {
            if (_useAlternatePeKey != value)
            {
                _useAlternatePeKey = value;
                _persianToEnglish = BuildPersianToEnglish(value);
            }
        }
    }

    private bool _useAlternatePeKey;

    private static Dictionary<char, char> BuildPersianToEnglish(bool alternatePeKey)
    {
        var map = new Dictionary<char, char>();
        foreach (var kv in EnglishToPersian)
        {
            if (kv.Value == 'پ')
            {
                continue; // «پ» به صورت جداگانه و بر اساس تنظیم مدیریت می‌شود
            }
            map.TryAdd(kv.Value, kv.Key);
        }
        map['پ'] = alternatePeKey ? '\\' : 'm';
        return map;
    }

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
            if (_persianToEnglish.TryGetValue(c, out char english))
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
