//! Get info about a map.

use crate::{
    config::Config,
    data::{map::parsed::map::MapData, stage::raw::stage_metadata::consts::StageTypeEnum},
};

/// Types of possible map.
enum Preset {
    // Main,
    Legend,
    Event,
    Gauntlet,
    Colosseum,
}

const fn get_preset(st: StageTypeEnum) -> Preset {
    type T = StageTypeEnum;
    match st {
        T::SoL | T::UL | T::ZL => Preset::Legend,
        T::Event | T::Collab => Preset::Event,
        T::Gauntlet | T::CollabGauntlet => Preset::Gauntlet,
        T::Colosseum => Preset::Colosseum,
        T::Dojo | T::RankingDojo | T::Championships => unimplemented!(),
        T::MainChapters | T::Outbreaks | T::Filibuster | T::AkuRealms => unimplemented!(),
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
pub fn get_map_info(map: &MapData, _config: &Config) -> String {
    println!("{map:#?}");
    todo!()
}
