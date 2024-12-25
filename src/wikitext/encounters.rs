//! Write out the encounters of an enemy.

pub mod chapter;
pub mod section;
use crate::{
    config::Config,
    data::{
        enemy::raw_encounters::stage_contains_enemy,
        stage::{
            get_stages,
            parsed::stage_enemy::StageEnemy,
            raw::{
                stage_data::StageData,
                stage_metadata::{consts::StageTypeEnum as T, StageMeta},
            },
        },
    },
    wikitext::{data_files::stage_page_data::STAGE_NAMES, wiki_utils::REGEXES},
};
use chapter::{Chapter, Group, Stage};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use order::enumerate_meta;
use section::{DisplayType, SectionRef};
use std::{borrow::Cow, fmt::Write};
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

fn key(meta: &StageMeta) -> (usize, u32, u32) {
    let m = match meta.type_enum {
        T::Extra => match STAGE_NAMES.continue_id(meta.map_num) {
            None => meta,
            Some((t, m)) => &StageMeta::from_numbers(t, m, 999).unwrap(),
        },
        _ => meta,
    };
    (enumerate_meta(m), m.map_num, m.stage_num)
}

fn sort_encounters(encounters: &mut [&StageData]) {
    encounters.sort_by(|s, o| key(&s.meta).cmp(&key(&o.meta)));
}

/// Get the section that the stage refers to.
///
/// Note: this does nothing about Removed Stages or any filtering based on type.
fn raw_section(meta: &StageMeta) -> SectionRef {
    match meta.type_enum {
        T::MainChapters => match meta.map_num {
            0..=2 => Ref::EoC,
            3..=5 => Ref::ItF,
            6..=8 => Ref::CotC,
            _ => unreachable!(),
        },
        T::Outbreaks => match meta.type_num {
            20 => Ref::EoCOutbreak,
            21 => Ref::ItFOutbreak,
            22 => Ref::CotCOutbreak,
            _ => unreachable!(),
        },
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

/// Get a mutable reference to the item in `group_chapters` that has the same
/// name as `stage_map`, creating it if it doesn't exist.
fn get_group_chapter<'a, 'b>(
    group_chapters: &'b mut Vec<Chapter<'a>>,
    map_name: Cow<'a, str>,
) -> &'b mut Chapter<'a>
where
    'a: 'b,
{
    if let Some(pos) = group_chapters
        .iter()
        .position(|c| c.chapter_name == map_name)
    {
        &mut group_chapters[pos]
    } else {
        let chap = Chapter::new(map_name, vec![]);
        group_chapters.push(chap);

        let i = group_chapters.len() - 1;
        &mut group_chapters[i]
    }
}

fn get_stage_mags(stage: &StageData, abs_enemy_id: u32) -> String {
    if stage.meta.type_enum == T::MainChapters && stage.meta.map_num == 0 {
        return "".to_string();
    };

    let mut mags = vec![];
    for enemy in stage.stage_csv_data.enemies.iter() {
        if enemy.num == abs_enemy_id {
            mags.push(StageEnemy::get_magnification(enemy))
        }
    }
    // fn enum_mag(mag: Magnification) -> (u8, u32) {
    //     match mag {
    //         Left(n) => (0, n),
    //         Right((n, _)) => (1, n),
    //     }
    // }
    // fn cmp_mag(m1: Magnification, m2: Magnification) -> Ordering {
    //     enum_mag(m1).cmp(&enum_mag(m2))
    // }

    mags.sort();
    mags.dedup();

    let mut buf = String::from("(");
    for mag in mags {
        match mag {
            Left(n) => {
                buf.write_formatted(&n, &Locale::en).unwrap();
                buf += "%";
            }
            Right(_) => todo!(),
        }

        buf += ", ";
    }
    buf.truncate(buf.len() - ", ".len());
    buf += ")";

    buf
}

/// Group section map into encounter [Groups][Group].
fn get_encounter_groups<'a>(
    sections_map: &[(SectionRef, Vec<&'a StageData<'_>>)],
    abs_enemy_id: u32,
) -> Vec<Group<'a>> {
    let mut removed = (Ref::Removed, vec![]);
    let mut groups: Vec<Group> = Vec::new();
    for map in sections_map {
        if map.1.is_empty() {
            continue;
        }
        let group = get_group(abs_enemy_id, map, &mut removed.1, true);
        groups.push(group);
    }
    if !removed.1.is_empty() {
        let map = removed;
        let group = get_group(abs_enemy_id, &map, &mut vec![], false);
        groups.push(group);
    }

    groups
}

#[inline]
fn get_group<'a: 'b, 'b>(
    abs_enemy_id: u32,
    section_map: &'b (SectionRef, Vec<&'a StageData<'a>>),
    removed_vec: &mut Vec<&'a StageData<'a>>,
    add_to_removed: bool,
) -> Group<'a> {
    let section = section_map.0.section();
    if *section.display_type() == DisplayType::Warn {
        eprintln!(
            "Warning: {:?} stages encountered.",
            section_map.1[0].meta.type_enum
        );
        // TODO log warning
    }

    let mut group = Group::new(section, vec![]);
    let group_chapters = &mut group.chapters;
    for stage in section_map.1.iter() {
        let stage_map = STAGE_NAMES
            .stage_map(stage.meta.type_num, stage.meta.map_num)
            .unwrap();

        if add_to_removed && REGEXES.old_or_removed_detect.is_match(&stage_map.name) {
            removed_vec.push(stage);
            continue;
        }
        if stage_map.name == "PLACEHOLDER" && stage_map.is_empty() {
            eprintln!(
                "Warning: map {:03}-{:03} is a placeholder.",
                stage.meta.type_num, stage.meta.map_num
            );
            // TODO warn macro
            continue;
        }

        let map_name = REGEXES
            .old_or_removed_sub
            .replace_all(&stage_map.name, "$1");
        let chap = get_group_chapter(group_chapters, map_name);

        let stage_name = match stage_map.get(stage.meta.stage_num) {
            Some(name) => name,
            None => {
                eprintln!(
                    "Warning: stage {:03}-{:03}-{:03} has no name.",
                    stage.meta.type_num, stage.meta.map_num, stage.meta.stage_num
                );
                // TODO warn macro
                continue;
            }
        };
        let mags = get_stage_mags(stage, abs_enemy_id);
        chap.stages
            .push(Stage::new(&stage_name.name, mags, &stage.meta));
    }
    group
}

/// Map [SectionRefs][SectionRef] to a list of [StageData].
fn get_section_map<'a>(
    encounters: &[&'a StageData<'a>],
) -> Vec<(SectionRef, Vec<&'a StageData<'a>>)> {
    let mut sections_map: Vec<(Ref, Vec<&StageData<'_>>)> = Vec::new();
    for encounter in encounters {
        let mut raw = raw_section(&encounter.meta);

        if *raw.section().display_type() == DisplayType::Skip {
            continue;
        }

        if raw == Ref::Extra {
            if let Some(ids) = STAGE_NAMES.continue_id(encounter.meta.map_num) {
                let new_meta = StageMeta::from_numbers(ids.0, ids.1, 999).unwrap();
                raw = raw_section(&new_meta);
            };
        }
        let raw = raw;

        if let Some(pos) = sections_map.iter().position(|(r, _)| *r == raw) {
            sections_map[pos].1.push(encounter);
        } else {
            sections_map.push((raw, vec![encounter]))
        };
    }
    sections_map
}

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let abs_enemy_id = wiki_id + 2;

    let all_stages = get_stages(&config.current_version).collect::<Vec<_>>();

    let mut encounters = all_stages
        .iter()
        .filter(|s| stage_contains_enemy(abs_enemy_id, s))
        .collect::<Vec<_>>();
    sort_encounters(&mut encounters);

    let section_map = get_section_map(&encounters);
    let groups = get_encounter_groups(&section_map, abs_enemy_id);

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
            // if chapter.stages.is_empty() {
            //     continue;
            // }
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
Due to how the encounters section is constantly evolving, this module cannot be
tested any other way than empirically.

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
