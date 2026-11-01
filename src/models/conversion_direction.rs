/// Direction of text conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionDirection {
    ToPersian,
    ToEnglish,
    None,
}

impl Default for ConversionDirection {
    fn default() -> Self {
        Self::None
    }
}

impl From<bool> for ConversionDirection {
    fn from(value: bool) -> Self {
        if value {
            Self::ToPersian
        } else {
            Self::ToEnglish
        }
    }
}
