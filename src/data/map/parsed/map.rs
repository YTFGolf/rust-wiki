//! Represents a map.

use crate::data::{stage::raw::{stage_data::StageData, stage_metadata::StageMeta}, version::Version};

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

pub fn do_thing(version:&Version) {
    let selector = "s 0";
    let selector_0 = selector.to_owned() + " 0";
    println!("{:?}", StageData::new(&selector_0, version))
    // log::debug!("{map_stage_data:#?}");
    // log::debug!("{map_option_data:#?}");
    // log::debug!("{ex_invasion:#?}");
    // log::debug!("{data:#?}");
}
