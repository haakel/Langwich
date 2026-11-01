/// Result of a text conversion operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionResult {
    pub original_text: String,
    pub converted_text: String,
    pub direction: ConversionDirection,
}

impl ConversionResult {
    pub fn new(original: &str, converted: &str, direction: ConversionDirection) -> Self {
        Self {
            original_text: original.to_string(),
            converted_text: converted.to_string(),
            direction,
        }
    }

    pub fn empty(text: &str) -> Self {
        Self {
            original_text: text.to_string(),
            converted_text: text.to_string(),
            direction: ConversionDirection::None,
        }
    }

    pub fn is_none(&self) -> bool {
        self.direction == ConversionDirection::None
    }

    pub fn is_to_persian(&self) -> bool {
        self.direction == ConversionDirection::ToPersian
    }

    pub fn is_to_english(&self) -> bool {
        self.direction == ConversionDirection::ToEnglish
    }
}

impl Default for ConversionResult {
    fn default() -> Self {
        Self::empty("")
    }
}
