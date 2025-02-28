//! ID for a stage.

use super::{
    map_id::{MapID, MapSize},
    variant::{StageVariant, VariantSize},
};

pub(super) type StageSize = u32;
/// Identifies a map.
pub struct StageID {
    map: MapID,
    num: StageSize,
}

// Simple methods on self.
impl StageID {
    /// Get stage type variant.
    pub fn variant(&self) -> StageVariant {
        self.map.variant()
    }

    /// Get stage map ID object.
    pub fn map(&self) -> &MapID {
        &self.map
    }

    /// Get map id used in game files.
    pub fn mapid(&self) -> u32 {
        self.map.mapid()
    }

    /// Get stage number.
    pub fn num(&self) -> StageSize {
        self.num
    }
}

// Initialisation.
impl StageID {
    /// Create new stage from components.
    pub fn from_components(variant: StageVariant, map: MapSize, num: StageSize) -> Self {
        Self::from_map(MapID::from_components(variant, map), num)
    }

    /// Create new stage from numbers.
    pub fn from_numbers(variant: VariantSize, map: MapSize, num: StageSize) -> Self {
        Self::from_components(variant.into(), map, num)
    }

    /// Create new stage from map.
    pub fn from_map(map: MapID, num: StageSize) -> Self {
        Self { map, num }
    }

    /// Create new stage from mapid.
    pub fn from_mapid(mapid: u32, num: StageSize) -> Self {
        Self::from_map(MapID::from_mapid(mapid), num)
    }
}
