//! ID for a stage.

use super::{map_id::MapID, variant::StageVariant};

type StageSize = u32;
/// Identifies a map.
pub struct StageID {
    map: MapID,
    num: StageSize,
}

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
