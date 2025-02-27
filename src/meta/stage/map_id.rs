//! ID for a stage map.

use super::variant::StageVariant;

/// Type of main chapter.
#[allow(missing_docs)]
pub enum MainType {
    EoC,
    ItF,
    CotC,
}

type MapSize = u32;
/// Identifies a map.
pub struct MapID {
    /// Stage type variant.
    variant: StageVariant,
    /// Number of map.
    num: MapSize,
}

impl MapID {
    /// Get stage type variant.
    pub fn variant(&self) -> StageVariant {
        self.variant
    }

    /// Get map number.
    pub fn num(&self) -> MapSize {
        self.num
    }

    /// Get map ID used in game files.
    pub fn mapid(&self) -> u32 {
        self.variant.num() * 1000 + self.num
    }

    /// If map is main chapters then get which one.
    ///
    /// # Panics
    ///
    /// Will panic if map number is not between 0 and 8.
    pub fn main_type_unchecked(&self) -> MainType {
        match self.num {
            (0..=2) => MainType::EoC,
            (3..=5) => MainType::ItF,
            (6..=8) => MainType::CotC,
            _ => panic!("Map num is not between 0 and 8."),
        }
    }
}
