//! Get info about a map.

mod event;
mod legend;
pub mod map_cli;
use crate::{
    config::Config,
    game_data::map::parsed::map::GameMap,
    game_data::meta::stage::{map_id::MapID, variant::StageVariantID},
};
use event::get_event_map;
use legend::get_legend_map;

/// battlecats-db reference.
pub fn reference(map: &MapID) -> String {
    let mapid = map.mapid();
    format!("https://battlecats-db.com/stage/s{mapid:05}.html")
}

/// Types of map that map info is implemented for.
enum Preset {
    // Main,
    Legend,
    Event,
    Gauntlet,
    Colosseum,
}

#[allow(clippy::match_same_arms)]
/// Get the preset value of the stage variant.
const fn get_preset(st: StageVariantID) -> Option<Preset> {
    type T = StageVariantID;
    match st {
        T::SoL | T::UL | T::ZL => Some(Preset::Legend),
        T::Event | T::Collab | T::Enigma => Some(Preset::Event),
        T::Gauntlet | T::CollabGauntlet => Some(Preset::Gauntlet),
        T::Colosseum => Some(Preset::Colosseum),
        T::Dojo | T::RankingDojo | T::Championships => None,
        //
        T::MainChapters | T::Filibuster | T::AkuRealms => None,
        //
        T::EocOutbreak | T::ItfOutbreak | T::CotcOutbreak => None,
        //
        T::Tower | T::Labyrinth => None,
        // One-time
        T::Behemoth => None,
        // ???
        T::Challenge => None,
        // Single stage
        T::Extra | T::Catamin => None,
        // No point
    }
}

/// Get full map info.
pub fn get_map_info(map: &GameMap, config: &Config) -> String {
    let preset = get_preset(map.id.variant()).unwrap();
    match preset {
        Preset::Legend => get_legend_map(map, config),
        Preset::Event => get_event_map(map, config),
        _ => todo!(),
    }
}
