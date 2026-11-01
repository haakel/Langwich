//! Core conversion engine for keyboard layout conversion
//!
//! Implements the exact same logic as the C# KeyboardLayoutConverterService.
//! Maps English characters to Persian based on Microsoft Persian keyboard layout.

use crate::models::{ConversionDirection, ConversionResult};
use std::collections::HashMap;
use unicode_normalization::UnicodeNormalization;

/// Main conversion engine
pub struct KeyboardLayoutConverter {
    /// English to Persian mapping (Microsoft Persian layout)
    english_to_persian: HashMap<char, char>,
    
    /// Persian to English mapping (rebuilt based on settings)
    persian_to_english: HashMap<char, char>,
    
    /// Whether to use alternate 'پ' key mapping (\ instead of m)
    use_alternate_pe_key: bool,
}

impl Default for KeyboardLayoutConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardLayoutConverter {
    /// Creates a new instance with default mappings
    pub fn new() -> Self {
        let english_to_persian = build_english_to_persian();
        let mut converter = Self {
            english_to_persian,
            persian_to_english: HashMap::new(),
            use_alternate_pe_key: false,
        };
        converter.rebuild_persian_to_english();
        converter
    }

    /// Creates English to Persian mapping based on Microsoft Persian layout
    fn build_english_to_persian() -> HashMap<char, char> {
        let mut map = HashMap::new();
        
        // Numbers row: 1234567890 → ۱۲۳۴۵۶۷۸۹۰
        map.insert('1', '۱');
        map.insert('2', '۲');
        map.insert('3', '۳');
        map.insert('4', '۴');
        map.insert('5', '۵');
        map.insert('6', '۶');
        map.insert('7', '۷');
        map.insert('8', '۸');
        map.insert('9', '۹');
        map.insert('0', '۰');
        map.insert('`', '٭');
        
        // Top row: qwertyuiop[] → ضصثغفغعحهخحچ
        map.insert('q', 'ض');
        map.insert('w', 'ص');
        map.insert('e', 'ث');
        map.insert('r', 'ق');
        map.insert('t', 'ف');
        map.insert('y', 'غ');
        map.insert('u', 'ع');
        map.insert('i', 'ه');
        map.insert('o', 'خ');
        map.insert('p', 'ح');
        map.insert('[', 'ج');
        map.insert(']', 'چ');
        
        // Home row: asdfghjkl;' → بشیلالانمکگ
        map.insert('a', 'ش');
        map.insert('s', 'س');
        map.insert('d', 'ی');
        map.insert('f', 'ب');
        map.insert('g', 'ل');
        map.insert('h', 'ا');
        map.insert('j', 'ت');
        map.insert('k', 'ن');
        map.insert('l', 'م');
        map.insert(';', 'ک');
        map.insert('\'', 'گ');
        
        // Bottom row: zxcvbnm,\ → ظطزرذدپو/
        map.insert('z', 'ظ');
        map.insert('x', 'ط');
        map.insert('c', 'ز');
        map.insert('v', 'ر');
        map.insert('b', 'ذ');
        map.insert('n', 'د');
        map.insert('m', 'پ');
        map.insert('\\', 'پ');
        map.insert(',', 'و');
        
        map
    }

    /// Rebuilds Persian to English mapping based on current settings
    fn rebuild_persian_to_english(&mut self) {
        self.persian_to_english.clear();
        
        // Reverse mapping for all characters except 'پ'
        for (eng, per) in &self.english_to_persian {
            if *per != 'پ' {
                self.persian_to_english.entry(*per).or_insert(*eng);
            }
        }
        
        // Set 'پ' mapping based on setting
        self.persian_to_english.insert('پ', if self.use_alternate_pe_key { '\\' } else { 'm' });
    }

    /// Converts text to Persian based on keyboard layout
    pub fn convert_to_persian(&self, english_text: &str) -> String {
        english_text
            .chars()
            .map(|c| {
                let lower_c = c.to_lowercase().next().unwrap_or(c);
                self.english_to_persian
                    .get(&lower_c)
                    .copied()
                    .unwrap_or(c)
            })
            .collect()
    }

    /// Converts text to English based on keyboard layout
    pub fn convert_to_english(&self, persian_text: &str) -> String {
        persian_text
            .chars()
            .map(|c| {
                self.persian_to_english
                    .get(&c)
                    .copied()
                    .unwrap_or(c)
            })
            .collect()
    }

    /// Performs automatic conversion direction detection and conversion
    pub fn convert(&self, text: &str) -> ConversionResult {
        if text.is_empty() {
            return ConversionResult::empty(text);
        }

        // Normalize Arabic/Persian characters
        let normalized = normalize_arabic_to_persian(text);

        // Count Persian vs Latin characters
        let persian_count = count_persian_letters(&normalized);
        let latin_count = count_latin_letters(&normalized);

        let (direction, converted) = if persian_count > 0 && persian_count >= latin_count {
            // Persian dominant → Convert to English
            (
                ConversionDirection::ToEnglish,
                self.convert_to_english(&normalized),
            )
        } else if latin_count > 0 && latin_count > persian_count {
            // Latin dominant → Convert to Persian
            (
                ConversionDirection::ToPersian,
                self.convert_to_persian(&normalized),
            )
        } else {
            // Equal or neither detected → No change
            (ConversionDirection::None, text.to_string())
        };

        ConversionResult::new(text, &converted, direction)
    }

    /// Sets whether to use alternate 'پ' key mapping
    pub fn set_use_persian_standard(&mut self, value: bool) {
        if self.use_alternate_pe_key != value {
            self.use_alternate_pe_key = value;
            self.rebuild_persian_to_english();
        }
    }

    /// Gets whether alternate 'پ' key is enabled
    pub fn use_persian_standard(&self) -> bool {
        self.use_alternate_pe_key
    }
}

/// Normalizes Arabic Unicode characters to Persian equivalents
fn normalize_arabic_to_persian(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\u0643' => 'ک', // Arabic Kaf → Persian Kaf
            '\u064A' => 'ی', // Arabic Ye → Persian Ye
            '\u0629' => 'ه', // Arabic Ta Marbuta → Heh
            '\u0621' => 'ء', // Arabic Hamza
            '\u0622' => 'آ', // Arabic Alaf with Madda
            '\u0623' => 'ا', // Arabic Alaf with Hamza above
            '\u0625' => 'ا', // Arabic Alaf with Hamza below
            '\u0624' => 'و', // Arabic Waw with Hamza
            '\u0626' => 'ی', // Arabic Ya with Hamza
            '\u0671' => 'ا', // Arabic Alaf Wasla
            _ => c,
        })
        .collect()
}

/// Counts Persian/Arabic letters in text
fn count_persian_letters(text: &str) -> usize {
    text.chars().filter(|c| is_persian_letter(*c)).count()
}

/// Counts Latin (ASCII) letters in text
fn count_latin_letters(text: &str) -> usize {
    text.chars()
        .filter(|c| {
            (c >= &'a' && c <= &'z') || (c >= &'A' && c <= &'Z')
        })
        .count()
}

/// Checks if a character is Persian/Arabic
fn is_persian_letter(c: char) -> bool {
    ('\u0600'..='\u06FF').contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sghl_to_salam() {
        let converter = KeyboardLayoutConverter::new();
        let result = converter.convert("sghl");
        assert_eq!(result.converted_text, "سلام");
        assert_eq!(result.direction, ConversionDirection::ToPersian);
    }

    #[test]
    fn test_to_engish() {
        let converter = KeyboardLayoutConverter::new();
        let result = converter.convert("سلام");
        assert_eq!(result.converted_text, "sghl");
        assert_eq!(result.direction, ConversionDirection::ToEnglish);
    }

    #[test]
    fn test_persian_standard_mapping() {
        let mut converter = KeyboardLayoutConverter::new();
        
        // Test default (m key)
        let result = converter.convert("پ");
        assert_eq!(result.converted_text, "m");
        
        // Test Persian standard (\ key)
        converter.set_use_persian_standard(true);
        let result = converter.convert("پ");
        assert_eq!(result.converted_text, "\\");
    }

    #[test]
    fn test_arabic_normalization() {
        let converter = KeyboardLayoutConverter::new();
        // Arabic ك should be normalized to Persian ک
        let result = converter.convert("كلمه");
        assert_eq!(result.converted_text, "kalm");
    }

    #[test]
    fn test_mixed_text_detection() {
        let converter = KeyboardLayoutConverter::new();
        
        // More Persian than English
        let result1 = converter.convert("سلام hi");
        assert_eq!(result1.direction, ConversionDirection::ToEnglish);
        
        // More English than Persian
        let result2 = converter.convert("hi سلام");
        assert_eq!(result2.direction, ConversionDirection::ToPersian);
    }
}
