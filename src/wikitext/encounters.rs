//! Write out the encounters of an enemy.

use crate::{
    config::Config,
    data::{
        enemy::raw_encounters::get_encounters,
        stage::raw::{
            stage_data::StageData,
            stage_metadata::{consts::StageTypeEnum as T, StageMeta},
        },
    },
};

const TYPE_ORDER: [T; 22] = [
    T::MainChapters,
    T::Outbreaks,
    T::Filibuster,
    T::AkuRealms,
    //
    T::SoL,
    T::UL,
    T::ZL,
    //
    T::Challenge,
    T::Event,
    T::Tower,
    T::Gauntlet,
    T::Behemoth,
    T::Colosseum,
    //
    T::Labyrinth,
    T::Collab,
    T::CollabGauntlet,
    T::Enigma,
    //
    T::Dojo,
    T::RankingDojo,
    T::Championships,
    //
    T::Catamin,
    T::Extra,
];

#[allow(clippy::zero_prefixed_literal)]
fn enumerate_meta(meta: &StageMeta) -> usize {
    TYPE_ORDER
        .iter()
        .position(|e| *e == meta.type_enum)
        .unwrap()
}

fn sort_encounters(encounters: Vec<StageData>) -> Vec<StageData<'_>> {
    let mut encounters = encounters;
    encounters.sort_by(|s, o| enumerate_meta(&s.meta).cmp(&enumerate_meta(&o.meta)));
    encounters
}

#[derive(Debug)]
/// How you display the section.
enum DisplayType {
    /// E.g. EoC: `*Stage x: name (mags)`.
    Flat,
    /// Standard `*map: stage` or `*map:\n**stage 1`.
    Normal,
    /// Main chapters; require extra logic.
    Custom,
    /// Format like Normal but give a warning to the user.
    Warn,
    /// Don't parse this at all.
    Skip,
}
type D = DisplayType;

#[derive(Debug)]
#[allow(dead_code)]
/// Section of unit encounters.
pub struct EncountersSection {
    heading: &'static str,
    display_type: DisplayType,
}

const fn get_new_section(heading: &'static str, display_type: DisplayType) -> EncountersSection {
    EncountersSection {
        heading,
        display_type,
    }
}

/// Removed stages. No point in being in [SECTIONS] because you need the stage
/// name for it.
pub const REMOVED_STAGES: EncountersSection =
    get_new_section("[[:Category:Removed Content|Removed Stages]]", D::Normal);
#[rustfmt::skip]
/// Available sections.
pub static SECTIONS: [EncountersSection; 17] = [
    get_new_section("[[Empire of Cats]]",                                    D::Custom),
    get_new_section("[[Empire of Cats]] [[Zombie Outbreaks|Outbreaks]]",     D::Custom),
    get_new_section("[[Into the Future]]",                                   D::Custom),
    get_new_section("[[Into the Future]] [[Zombie Outbreaks|Outbreaks]]",    D::Custom),
    get_new_section("[[Cats of the Cosmos]]",                                D::Custom),
    get_new_section("[[Cats of the Cosmos]] [[Zombie Outbreaks|Outbreaks]]", D::Custom),
    get_new_section("[[The Aku Realms]]",                                    D::Custom),

    get_new_section("[[Legend Stages#Stories of Legend|Stories of Legend]]", D::Flat),
    get_new_section("[[Legend Stages#Uncanny Legends|Uncanny Legends]]",     D::Flat),
    get_new_section("[[Legend Stages#Zero Legends|Zero Legends]]",           D::Flat),

    get_new_section("[[Special Events|Event Stages]]",                       D::Normal),
    get_new_section("[[Underground Labyrinth]]",                             D::Flat),
    get_new_section("[[Collaboration Event Stages|Collaboration Stages]]",   D::Normal),
    get_new_section("[[Enigma Stages]]",                                     D::Normal),
    get_new_section("[[Catclaw Dojo]]",                                      D::Normal),

    get_new_section("Extra Stages",                                          D::Warn),
    get_new_section("[[Catamin Stages]]",                                    D::Skip),
];

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let abs_enemy_id = wiki_id + 2;
    let encounters = get_encounters(abs_enemy_id, &config.current_version);
    let encounters = sort_encounters(encounters);
    println!("{:?}", encounters);
    println!("{SECTIONS:?}");
}

/*
# Flow
## Wikitext
- Order stages + sort out extra stages
    - Order is done by a Rust sort
    - Extra stages will be done with... something idk. Setting to 999 should work
      since if a stage is an earlier continuation then it would just appear before
      the later ones. Would also fix like proving ground continuations.
- Loop through sections:
    - Get stage names for each stage
    - Display stage names. Filter out if doesn't begin with '['.
        - Hashmap for map name display type
        - Vec for stage display type
- If Catamin or extra stages then should print dire warning
- Else copy to clipboard, message saying "copied to clipboard" in green

Other things:
- StageData::new; StageEnemy::get_magnification
- Some logging crate needed to log out which pages are skipped
- Testing can be done easily for small parts but the overall thing can only be
  measured empirically
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::stage::raw::stage_metadata::consts::STAGE_TYPES;

    #[test]
    fn test_type_order() {
        assert_eq!(STAGE_TYPES.len(), TYPE_ORDER.len());
        for stype in STAGE_TYPES {
            assert!(
                TYPE_ORDER.contains(&stype.type_enum),
                "Type order array does not contain variant {:?}",
                stype.type_enum
            );
        }
    }
}
