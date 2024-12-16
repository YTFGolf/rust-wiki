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
    wikitext::data_files::stage_page_data::STAGE_NAMES,
};
use chapter::{Chapter, Group, Stage};
use order::enumerate_meta;
use section::{DisplayType, SectionRef};
use std::fmt::Write;
type Ref = SectionRef;

mod order {
    use crate::data::stage::raw::stage_metadata::{
        consts::{StageTypeEnum as T, STAGE_TYPES},
        StageMeta,
    };

    /// Amount of individual [StageTypes][T] (count is based on enums).
    const STYPE_AMT: usize = STAGE_TYPES.len();

    /// Order of the [StageTypes][T] in Encounters section.
    const TYPE_ORDER: [T; STYPE_AMT] = [
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

    /// Convert [TYPE_ORDER] to its indices. Allows [enumerate_meta] to be a
    /// constant time function.
    const fn get_type_order() -> [usize; STYPE_AMT] {
        let mut sum = [0; STYPE_AMT];

        let mut i = 0;
        // for loops and iterators don't work in constant functions,
        // but while loops do
        while i < STYPE_AMT {
            let t = TYPE_ORDER[i] as usize;
            sum[t] = i;
            i += 1;
        }

        sum
    }

    /// Order indices using the [usize] value of [StageTypeEnum][T].
    ///
    /// For example, since [T::MainChapters] is first in [TYPE_ORDER], indexing
    /// `TYPE_ORDER_INDICES[T::MainChapters as usize]` would yield `0`.
    const TYPE_ORDER_INDICES: [usize; STYPE_AMT] = get_type_order();
    // doctest would cause visibility nightmares so just use const assert
    const _: () = assert!(TYPE_ORDER_INDICES[T::MainChapters as usize] == 0);

    /// Enumerate a [StageMeta] object for use in comparisons.
    pub const fn enumerate_meta(meta: &StageMeta) -> usize {
        TYPE_ORDER_INDICES[meta.type_enum as usize]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
}

fn sort_encounters(encounters: Vec<StageData>) -> Vec<StageData<'_>> {
    let mut encounters = encounters;
    encounters.sort_by(|s, o| enumerate_meta(&s.meta).cmp(&enumerate_meta(&o.meta)));
    encounters
}

/// Get the section that the stage refers to.
///
/// Note: this does nothing about Removed Stages or any filtering based on type.
fn raw_section(meta: &StageMeta) -> SectionRef {
    match meta.type_enum {
        T::MainChapters => {
            todo!()
        }
        T::Outbreaks => {
            todo!()
        }
        T::Filibuster => Ref::CotC,
        T::AkuRealms => Ref::AkuRealms,
        T::SoL => Ref::SoL,
        T::UL => Ref::UL,
        T::ZL => Ref::ZL,
        T::Event | T::Tower | T::Challenge | T::Gauntlet | T::Behemoth | T::Colosseum => Ref::Event,
        T::Labyrinth => Ref::Labyrinth,
        T::Collab | T::CollabGauntlet => Ref::Collab,
        T::Enigma => Ref::Enigma,
        T::Dojo | T::RankingDojo | T::Championships => Ref::Dojo,
        T::Extra => Ref::Extra,
        T::Catamin => Ref::Catamin,
    }
}

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let abs_enemy_id = wiki_id + 2;
    let encounters = get_encounters(abs_enemy_id, &config.current_version);
    let encounters = sort_encounters(encounters);

    // println!("{:?}", encounters);

    // iterate
    // if skip continue
    // let a = Ref::AkuRealms;
    // a.section();
    let mut sections_map: Vec<(Ref, Vec<StageData<'_>>)> = Vec::new();

    for encounter in encounters {
        let raw = raw_section(&encounter.meta);
        let section = raw.section();

        if *section.display_type() == DisplayType::Skip {
            continue;
        }

        // - [ ] if extra use continuestages to find actual place

        if let Some(pos) = sections_map.iter().position(|(r, _)| *r == raw) {
            sections_map[pos].1.push(encounter);
        } else {
            sections_map.push((raw, vec![encounter]))
        };
    }

    // let removed: (Ref, Vec<StageData<'_>>) = (Ref::Removed, vec![]);
    let mut groups: Vec<Group> = Vec::new();
    for map in sections_map.iter() {
        if map.1.is_empty() {
            continue;
        }

        let section = map.0.section();
        if *section.display_type() == DisplayType::Warn {
            eprintln!("Warning: {:?} stages encountered.", map.1[0].meta.type_enum);
            // TODO log warning
        }

        let mut group = Group::new(section, vec![]);
        let group_chapters = &mut group.chapters;
        for stage in map.1.iter() {
            let stage_map = STAGE_NAMES
                .stage_map(stage.meta.type_num, stage.meta.map_num)
                .unwrap();
            // Add to removed

            let chap = if let Some(pos) = group_chapters
                .iter()
                .position(|c| c.chapter_name == stage_map.name)
            {
                &mut group_chapters[pos]
            } else {
                let chap = Chapter::new(&stage_map.name, vec![]);
                group_chapters.push(chap);

                let i = group_chapters.len() - 1;
                &mut group_chapters[i]
            };

            let stage_name = stage_map.get(stage.meta.stage_num).unwrap();
            // TODO
            let mags = "".to_string();
            chap.stages
                .push(Stage::new(&stage_name.name, mags, &stage.meta));
        }

        groups.push(group);
    }

    let mut buf = String::from("==Encounters==\n{{Collapsible}}");
    for group in groups {
        if group.chapters.is_empty() {
            continue;
        }

        write!(
            &mut buf,
            "\n==={heading}===\n",
            heading = group.section.heading()
        )
        .unwrap();

        for chapter in group.chapters {
            group.section.fmt_chapter(&mut buf, chapter);
            buf += "\n";
        }
    }
    buf += "</div>";

    println!("{buf}");

    /*
    - [x] get
    - [x] sort
    - [x] iterate
      - [x] find section
        - [x] if Skip then skip
      - [x] add to section list
    - [x] Go through each section
      - [x] Warn about extra stages
      - [x] find stage name
      - [ ] filter/move to removed stages
      - [x] for each chapter:
        - [ ] remove dupes
        - [ ] get mags
        - [x] format section

    ## extensions
    - [ ] analyse all stages to see if has same mag in all
    - [ ] analyse eoc outbreaks
    */
}

// Encounter name filter or something
// Remove all catamin stages
// move removed to section
// eliminate unlinked stages and warn
// move extra stages into correct section
// remove princess punt eoc stages
// from stage meta get heading
// removed is done just by string search

/*
# Flow
## Wikitext
- Order stages + sort out extra stages
    - Order is done by a Rust sort
    - Extra stages will be done with... something idk. Setting to 999 should work
      since if a stage is an earlier continuation then it would just appear before
      the later ones.
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
