//! Get info about a map.

mod legend;
use crate::{
    config::Config,
    data::map::parsed::map::MapData,
    meta::stage::{map_id::MapID, variant::StageVariantID},
};
use legend::get_legend_map;

/// Types of possible map.
enum Preset {
    // Main,
    Legend,
    Event,
    Gauntlet,
    Colosseum,
}

const fn get_preset(st: StageVariantID) -> Preset {
    type T = StageVariantID;
    match st {
        T::SoL | T::UL | T::ZL => Preset::Legend,
        T::Event | T::Collab => Preset::Event,
        T::Gauntlet | T::CollabGauntlet => Preset::Gauntlet,
        T::Colosseum => Preset::Colosseum,
        T::Dojo | T::RankingDojo | T::Championships => unimplemented!(),
        //
        T::MainChapters | T::Filibuster | T::AkuRealms => unimplemented!(),
        //
        T::EocOutbreak | T::ItfOutbreak | T::CotcOutbreak => unimplemented!(),
        //
        T::Tower | T::Labyrinth => unimplemented!(),
        // One-time
        T::Behemoth => unimplemented!(),
        // ???
        T::Challenge | T::Enigma => unimplemented!(),
        // Single stage per
        T::Extra | T::Catamin => unimplemented!(),
        // No point
    }
}

/// Get full map info.
pub fn get_map_info(map: &MapData, config: &Config) -> String {
    let map_id: MapID = (&map.meta).into();
    let preset = get_preset(map_id.variant());
    match preset {
        Preset::Legend => get_legend_map(map, config),
        _ => todo!(),
    }
}
