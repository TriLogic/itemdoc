use super::items::ItemType;
use super::booleans::ItemBoolean;
use super::numbers::ItemNumber;
use super::strings::ItemString;

#[derive(Debug, Clone, PartialEq)]
pub enum RustType {
    Bool(bool),
    Number(f64),
    String(String),
}

impl From<bool> for RustType {
    fn from(b: bool) -> Self {
        RustType::Bool(b)
    }
}

impl From<f64> for RustType {
    fn from(n: f64) -> Self {
        RustType::Number(n)
    }
}

impl From<&str> for RustType {
    fn from(s: &str) -> Self {
        RustType::String(s.to_string())
    }
}

impl From<String> for RustType {
    fn from(s: String) -> Self {
        RustType::String(s)
    }
}

impl RustType {
    pub fn into_item_type(self) -> ItemType {
        match self {
            RustType::Bool(b) => ItemBoolean::new(Some(b)),
            RustType::Number(n) => ItemNumber::new(Some(n)),
            RustType::String(s) => ItemString::new(Some(s)),
        }
    }
}

