//! Represents a map.

use std::{num::NonZeroU32, thread::panicking};

use crate::data::{
    map::{map_data::GameMap, special_rules::SpecialRule},
    stage::{
        parsed::stage::{CrownData, Restriction, RestrictionStages},
        raw::stage_metadata::StageMeta,
    },
    version::Version,
};

#[derive(Debug)]
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
    /// Easier to just reuse StageMeta.
    pub meta: StageMeta,
    // Map option
    pub crown_data: Option<CrownData>,
    reset_type: ResetType,
    max_clears: Option<NonZeroU32>,
    _display_order: Option<u32>,
    cooldown: Option<NonZeroU32>,
    star_mask: Option<u16>,
    hidden_upon_clear: bool,
    //
    restrictions: Option<Vec<Restriction>>,
    ex_option_map: Option<u32>,
    special_rule: Option<SpecialRule>,
}
impl MapData {
    fn new(mapid: u32, version: &Version) -> Self {
        let type_id = mapid / 1000;
        let map_id = mapid % 1000;
        let m = StageMeta::from_numbers(type_id, map_id, 0).unwrap();
        Self::from_meta(m, version)
    }

    fn from_meta(m: StageMeta, version: &Version) -> Self {
        let map_option_data = GameMap::get_map_option_data(&m, version);

        let crown_data: Option<CrownData>;
        let reset_type: ResetType;
        let max_clears: Option<NonZeroU32>;
        let _display_order: Option<u32>;
        let cooldown: Option<NonZeroU32>;
        let star_mask: Option<u16>;
        let hidden_upon_clear: bool;

        if let Some(data) = map_option_data {
            crown_data = Some(CrownData::from(&data));
            reset_type = ResetType::from(data.reset_type);
            max_clears = NonZeroU32::new(data.max_clears);
            _display_order = Some(data._display_order);
            cooldown = NonZeroU32::new(data.cooldown);
            star_mask = Some(data.star_mask);
            hidden_upon_clear = u8_to_bool(data.hidden_upon_clear);
        } else {
            crown_data = None;
            reset_type = ResetType::None;
            max_clears = None;
            _display_order = None;
            cooldown = None;
            star_mask = None;
            hidden_upon_clear = false;
        }

        let restrictions: Option<Vec<Restriction>>;
        if let Some(option_data) = GameMap::get_stage_option_data(&m, version) {
            restrictions = Some(
                option_data
                    .into_iter()
                    .filter_map(|r| {
                        let r = Restriction::from_option_csv(r, version);
                        match r.stage {
                            RestrictionStages::All => Some(r),
                            RestrictionStages::One(_) => None,
                        }
                    })
                    .collect(),
            );
        } else {
            restrictions = None;
        }

        let ex_option_map = GameMap::get_ex_option_data(&m, version);
        let special_rule = GameMap::get_special_rules_data(&m, version).cloned();
        Self {
            meta: m,
            //
            crown_data,
            reset_type,
            max_clears,
            _display_order,
            cooldown,
            star_mask,
            hidden_upon_clear,
            //
            restrictions,
            ex_option_map,
            special_rule,
        }
    }
}
const fn u8_to_bool(u: u8) -> bool {
    match u {
        0 => false,
        1 => true,
        _ => panic!("Not a valid boolean value!"),
    }
}

pub fn do_thing(version: &Version) {
    let mapids = vec![0, 1000, 1209, 1237, 1385, 36004, 24035];
    for mapid in mapids {
        let type_id = mapid / 1000;
        let map_id = mapid % 1000;
        let m = StageMeta::from_numbers(type_id, map_id, 0).unwrap();

        log::debug!("{:#?}", MapData::from_meta(m, version));
        println!("");
    }

    panic!("aaaah")
}
