//! ID for a stage map.

use super::variant::{StageVariantID, VariantSize};
use std::fmt::Display;

/// Type of main chapter.
pub enum MainType {
    /// EoC.
    EoC,
    /// ItF.
    ItF,
    /// CotC.
    CotC,
}

/// Size of map number.
pub type MapSize = u32;
#[derive(Debug, PartialEq, Clone)]
/// Identifies a map.
pub struct MapID {
    /// Stage type variant.
    variant: StageVariantID,
    /// Number of map.
    num: MapSize,
}

// Simple methods on self.
impl MapID {
    /// Get stage type variant.
    pub const fn variant(&self) -> StageVariantID {
        self.variant
    }

    /// Get map number.
    pub const fn num(&self) -> MapSize {
        self.num
    }

    /// Get map ID used in game files.
    pub const fn mapid(&self) -> u32 {
        self.variant.num() * 1000 + self.num
    }

    /// If map is main chapters then get which one if applicable.
    pub const fn main_type(&self) -> Option<MainType> {
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
    /// Create new [`MapID`] from components.
    pub const fn from_components(variant: StageVariantID, num: MapSize) -> Self {
        Self { variant, num }
    }

    /// Create new [`MapID`] from numbers.
    pub fn from_numbers(variant: VariantSize, num: MapSize) -> Self {
        Self::from_components(variant.into(), num)
    }

    /// Create new [`MapID`] from mapid.
    pub fn from_mapid(mapid: u32) -> Self {
        let type_id = mapid / 1000;
        let map_id = mapid % 1000;
        Self::from_numbers(type_id, map_id)
    }
}

// Mutation.
impl MapID {
    /// Set map ID number.
    pub fn set_num(&mut self, num: MapSize) {
        self.num = num;
    }
}

impl Display for MapID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:03}-{:03}", self.variant.num(), self.num)
    }
}
