//! ID for a stage map.

use super::variant::{StageVariant, VariantSize};

/// Type of main chapter.
#[allow(missing_docs)]
pub enum MainType {
    EoC,
    ItF,
    CotC,
}

pub(super) type MapSize = u32;
/// Identifies a map.
pub struct MapID {
    /// Stage type variant.
    variant: StageVariant,
    /// Number of map.
    num: MapSize,
}

// Simple methods on self.
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

    /// If map is main chapters then get which one if applicable.
    pub fn main_type(&self) -> Option<MainType> {
        match self.num {
            (0..=2) => Some(MainType::EoC),
            (3..=5) => Some(MainType::ItF),
            (6..=8) => Some(MainType::CotC),
            _ => None,
        }
    }
}

// Initialisation.
impl MapID {
    /// Create new MapID from components.
    pub fn from_components(variant: StageVariant, num: MapSize) -> Self {
        Self { variant, num }
    }

    /// Create new MapID from numbers.
    pub fn from_numbers(variant: VariantSize, num: MapSize) -> Self {
        Self::from_components(variant.into(), num)
    }

    /// Create new MapID from mapid.
    pub fn from_mapid(mapid: u32) -> Self {
        let type_id = mapid / 1000;
        let map_id = mapid % 1000;
        Self::from_numbers(type_id, map_id)
    }
}
