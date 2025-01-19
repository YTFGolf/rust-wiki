//! Represents a map.

use crate::data::{
    map::map_data::GameMap, stage::raw::stage_metadata::StageMeta, version::Version,
};

/// What happens when event ends. Event can be ended by reaching max clears or
/// by the timer running out.
pub enum ResetType {
    /// Do nothing on reset.
    None = 0,
    /// Reset rewards. Used in events like B-Day Gifts, as well as XP Bonanza.
    ResetRewards = 1,
    /// Reset rewards and map clearance.
    ///
    /// Used in most Gauntlets, the Towers, and generally most limited events
    /// (especially Collabs).
    ResetRewardsAndClear = 2,
    /// Reset only max clears.
    ///
    /// E.g. Behemoth Culling, Proving Grounds, Catamin stages.
    ResetMaxClears = 3,
    // reward1=Rewards are reset per appearance
    // reward2=Clear status and rewards are reset per appearance
    // reward3=Number of plays are reset per appearance
}
impl From<u8> for ResetType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::ResetRewards,
            2 => Self::ResetRewardsAndClear,
            3 => Self::ResetMaxClears,
            _ => panic!("Reset type not recognised: {value}."),
        }
    }
}

#[derive(Debug)]
struct MapData {
    // Map option
    // Stage option
    // EX option
    ex_option_map: Option<u32>,
    // Special rules
}
impl MapData {
    fn new(mapid: u32, version: &Version) -> Self {
        let type_id = mapid / 1000;
        let map_id = mapid % 1000;
        let m = StageMeta::from_numbers(type_id, map_id, 0).unwrap();
        Self::from_meta(&m, version)
    }

    fn from_meta(m: &StageMeta, version: &Version) -> Self {
        let ex_option_map = GameMap::get_ex_option_data(&m, version);
        Self { ex_option_map }
    }
}

pub fn do_thing(version: &Version) {
    let mapids = vec![0, 1000, 1209, 1385];
    for mapid in mapids {
        let type_id = mapid / 1000;
        let map_id = mapid % 1000;
        let m = StageMeta::from_numbers(type_id, map_id, 0).unwrap();

        log::debug!(
            "GameMap::get_map_option_data: {data:#?}",
            data = GameMap::get_map_option_data(&m, version)
        );
        log::debug!(
            "GameMap::get_stage_option_data: {data:#?}",
            data = GameMap::get_stage_option_data(&m, version)
        );
        log::debug!(
            "GameMap::get_special_rules_data: {data:#?}",
            data = GameMap::get_special_rules_data(&m, version)
        );

        log::debug!("{:#?}", MapData::from_meta(&m, version));
    }

    panic!("aaaah")
}
