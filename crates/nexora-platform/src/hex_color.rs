use serde::{Deserialize, Deserializer, Serialize};

/// Exact RGB display metadata, normalized to uppercase at input boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct HexColor(String);
impl HexColor {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value.len() != 7
            || !value.starts_with('#')
            || !value.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit)
        {
            return Err(serde::de::Error::custom(
                "color must use #RRGGBB hexadecimal format",
            ));
        }
        Ok(Self(value.to_ascii_uppercase()))
    }
}
