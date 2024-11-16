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

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let abs_enemy_id = wiki_id + 2;
    let encounters = get_encounters(abs_enemy_id, &config.current_version);
    println!("{:?}", sort_encounters(encounters));
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

Classes:
- EncountersSection enum: contains ordering and initialisation as well.
- DisplayType enum: `Stage x` or map name

Other things:
- StageData::new; StageEnemy::get_magnification
- Some logging crate needed to log out which pages are skipped
- Testing can be done easily for small parts but the overall thing can only be
  measured empirically
*/
