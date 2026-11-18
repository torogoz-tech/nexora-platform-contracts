use crate::HexColor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// An omitted field preserves metadata; explicit null clears it.
#[derive(Debug, Clone, Default)]
pub enum ColorChange {
    #[default]
    Unchanged,
    Clear,
    Set(HexColor),
}
impl ColorChange {
    pub fn is_unchanged(&self) -> bool {
        matches!(self, Self::Unchanged)
    }
    pub fn apply(&self, current: Option<HexColor>) -> Option<HexColor> {
        match self {
            Self::Unchanged => current,
            Self::Clear => None,
            Self::Set(color) => Some(color.clone()),
        }
    }
}
impl<'de> Deserialize<'de> for ColorChange {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Option::<HexColor>::deserialize(deserializer)?.map_or(Self::Clear, Self::Set))
    }
}
impl Serialize for ColorChange {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Set(color) => color.serialize(serializer),
            Self::Clear | Self::Unchanged => serializer.serialize_none(),
        }
    }
}
