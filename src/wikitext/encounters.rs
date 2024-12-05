//! Write out the encounters of an enemy.

pub mod chapter;
pub mod section;
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
use chapter::{Chapter, Stage};
use section::{REMOVED_STAGES, SECTIONS};

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

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let mut buf = String::from("");
    REMOVED_STAGES.fmt_chapter(
        &mut buf,
        Chapter::new(
            "Chapter",
            &[
                Stage::new("Stage 1", "(100%)", &StageMeta::new("event 0 0").unwrap()),
                Stage::new("Stage 2", "", &StageMeta::new("event 0 1").unwrap()),
            ],
        ),
    );
    println!("{buf}");

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
