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

#[allow(clippy::zero_prefixed_literal)]
fn enumerate_meta(meta: &StageMeta) -> u32 {
    match meta.type_enum {
        T::Extra => 001,
        _ => 000,
    }
}

fn sort_encounters(encounters: Vec<StageData>) -> Vec<StageData<'_>> {
    let mut encounters = encounters;
    encounters.sort_by(|s, o| enumerate_meta(&s.meta).cmp(&enumerate_meta(&o.meta)));
    encounters
}

enum DisplayType {
    /// E.g. EoC: `*Stage x: name (mags)`.
    Flat,
    /// Standard `*map: stage` or `*map:\n**stage 1`.
    Normal,
    /// Format like Normal but give a warning to the user.
    Warn,
    /// Don't parse this at all.
    Skip,
}
type D = DisplayType;
struct EncountersSection {
    heading: &'static str,
    display_type: DisplayType,
}
const fn get_new_section(heading: &'static str, display_type: DisplayType) -> EncountersSection {
    EncountersSection {
        heading,
        display_type,
    }
}

#[rustfmt::skip]
static SECTIONS: [EncountersSection; 18] = [
    get_new_section("[[Empire of Cats]]",                                    D::Flat),
    get_new_section("[[Empire of Cats]] [[Zombie Outbreaks|Outbreaks]]",     D::Flat),
    get_new_section("[[Into the Future]]",                                   D::Flat),
    get_new_section("[[Into the Future]] [[Zombie Outbreaks|Outbreaks]]",    D::Flat,),
    get_new_section("[[Cats of the Cosmos]]",                                D::Flat),
    get_new_section("[[Cats of the Cosmos]] [[Zombie Outbreaks|Outbreaks]]", D::Flat,),
    get_new_section("[[The Aku Realms]]",                                    D::Flat),

    get_new_section("[[Legend Stages#Stories of Legend|Stories of Legend]]", D::Flat,),
    get_new_section("[[Legend Stages#Uncanny Legends|Uncanny Legends]]",     D::Flat),
    get_new_section("[[Legend Stages#Zero Legends|Zero Legends]]",           D::Flat),

    get_new_section("[[Special Events|Event Stages]]",                       D::Normal),
    get_new_section("[[Underground Labyrinth]]",                             D::Flat),
    get_new_section("[[Collaboration Event Stages|Collaboration Stages]]",   D::Normal,),
    get_new_section("[[Enigma Stages]]",                                     D::Normal),
    get_new_section("[[Catclaw Dojo]]",                                      D::Normal),
    get_new_section("[[:Category:Removed Content|Removed Stages]]",          D::Normal),

    get_new_section("Extra Stages",                                          D::Warn),
    get_new_section("[[Catamin Stages]]",                                    D::Skip),
];

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let abs_enemy_id = wiki_id + 2;
    let encounters = get_encounters(abs_enemy_id, &config.current_version);
    let encounters = sort_encounters(encounters);
    println!("{:?}", encounters);
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
