//! ID for a stage.

use super::{
    map_id::{MapID, MapSize},
    variant::{StageVariantID, VariantSize},
};
use std::fmt::Display;

/// Size of stage number.
pub type StageSize = u32;
#[derive(Debug, PartialEq, Clone)]
/// Identifies a stage.
pub struct StageID {
    map: MapID,
    num: StageSize,
}

// Simple methods on self.
impl StageID {
    /// Get stage type variant.
    pub const fn variant(&self) -> StageVariantID {
        self.map.variant()
    }

    /// Get stage map ID object.
    pub const fn map(&self) -> &MapID {
        &self.map
    }

    /// Get stage number.
    pub const fn num(&self) -> StageSize {
        self.num
    }
}

// Initialisation.
impl StageID {
    /// Create new stage from components.
    pub const fn from_components(variant: StageVariantID, map: MapSize, num: StageSize) -> Self {
        Self::from_map(MapID::from_components(variant, map), num)
    }

    /// Create new stage from numbers.
    pub fn from_numbers(variant: VariantSize, map: MapSize, num: StageSize) -> Self {
        Self::from_components(variant.into(), map, num)
    }

    /// Create new stage from map.
    pub const fn from_map(map: MapID, num: StageSize) -> Self {
        Self { map, num }
    }

    /// Create new stage from mapid.
    pub fn from_mapid(mapid: u32, num: StageSize) -> Self {
        Self::from_map(MapID::from_mapid(mapid), num)
    }
}

// Mutation.
impl StageID {
    /// Set map ID number.
    pub fn set_map(&mut self, map: StageSize) {
        self.map.set_num(map);
    }

    /// Set stage ID number.
    pub fn set_num(&mut self, num: StageSize) {
        self.num = num;
    }
}

impl Display for StageID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:03}-{:03}-{:03}",
            self.variant().num(),
            self.map().num(),
            self.num
        )
    }
}
