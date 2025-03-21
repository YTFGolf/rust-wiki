//! Get info about a map.

mod legend;
use crate::{
    config::Config, data::map::parsed::map::GameMap, meta::stage::variant::StageVariantID,
};
use legend::get_legend_map;

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
        T::Event | T::Collab => Some(Preset::Event),
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
        T::Challenge | T::Enigma => None,
        // Single stage per
        T::Extra | T::Catamin => None,
        // No point
    }
}

/// Get full map info.
pub fn get_map_info(map: &GameMap, config: &Config) -> String {
    let preset = get_preset(map.id.variant()).unwrap();
    match preset {
        Preset::Legend => get_legend_map(map, config),
        _ => todo!(),
    }
}
