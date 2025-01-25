//! Write out the encounters of an enemy.

pub mod chapter;
pub mod section;
pub mod zoutbreak;
use crate::{
    config::Config,
    data::{
        enemy::raw_encounters::stage_contains_enemy,
        stage::{
            get_stages,
            parsed::stage_enemy::StageEnemy,
            raw::{
                stage_data::StageData,
                stage_metadata::{consts::StageTypeEnum as T, StageMeta, StageMetaParseError},
            },
        },
    },
    wikitext::{
        data_files::stage_wiki_data::STAGE_WIKI_DATA,
        wiki_utils::{OLD_OR_REMOVED_DETECT, OLD_OR_REMOVED_SUB},
    },
};
use chapter::{Chapter, Group, Stage};
use either::Either::{Left, Right};
use num_format::{Locale, WriteFormatted};
use order::enumerate_meta;
use regex::Regex;
use section::{DisplayType, SectionRef};
use std::{borrow::Cow, collections::HashSet, fmt::Write};
use zoutbreak::manual_zoutbreak_replace;
type Ref = SectionRef;

mod order {
    use crate::data::stage::raw::stage_metadata::{
        consts::{StageTypeEnum as T, STAGE_TYPES},
        StageMeta,
    };

    /// Amount of individual [StageTypes][T] contained in [STAGE_TYPES].
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
        let mut order_indices = [0; STYPE_AMT];

        let mut i = 0;
        // for loops and iterators don't work in constant functions,
        // but while loops do
        while i < STYPE_AMT {
            let enum_value = TYPE_ORDER[i] as usize;
            order_indices[enum_value] = i;
            i += 1;
        }

        order_indices
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
        /// Ensure type order has all [STAGE_TYPES].
        fn test_type_order() {
            // technically this could be converted to a const assert but why
            // bother.
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

/// For use in [sort_encounters].
fn key(meta: &StageMeta) -> (usize, u32, u32) {
    let m = match meta.type_enum {
        T::Extra => match STAGE_WIKI_DATA.continue_id(meta.map_num) {
            None => meta,
            Some((t, m)) => match t {
                30 => &StageMeta::from_selector_main(&t.to_string(), &[999]).unwrap(),
                _ => &StageMeta::from_numbers(t, m, 999).unwrap(),
            },
            // TODO put unit tests in for mount aku
        },
        // Obviously want extra stages to be in correct place if possible
        _ => meta,
    };
    (enumerate_meta(m), m.map_num, m.stage_num)
    // extra stages will need the map (and possibly stage idk) num to be in the
    // correct place
    // TODO check if stage num is necessary

    // Type num is not necessary since it will only make a difference for
    // outbreaks, which each have their own section anyway.
}

/// Sort `encounters` in-place.
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
/// chapter name as `map_name`, creating it if it doesn't exist.
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
        return &mut group_chapters[pos];
    }

    let chap = Chapter::new(map_name, vec![]);
    group_chapters.push(chap);

    let i = group_chapters.len() - 1;
    &mut group_chapters[i]
}

/// Get a list of magnifications the enemy appears at.
fn get_stage_mags(stage: &StageData, abs_enemy_id: u32) -> String {
    if stage.meta.type_enum == T::MainChapters && stage.meta.map_num == 0 {
        return String::new();
    };

    let mut mags = vec![];
    for enemy in &stage.stage_csv_data.enemies {
        if enemy.num == abs_enemy_id {
            mags.push(StageEnemy::get_magnification(enemy));
        }
    }

    mags.sort();
    // Order is Left > Right and then for any Right variant it checks hp then ap
    mags.dedup();

    let mut buf = String::from("(");
    for mag in mags {
        match mag {
            Left(n) => {
                buf.write_formatted(&n, &Locale::en).unwrap();
                buf += "%";
            }
            Right((hp, ap)) => {
                buf.write_formatted(&hp, &Locale::en).unwrap();
                buf += "% HP/";
                buf.write_formatted(&ap, &Locale::en).unwrap();
                buf += "% AP";
            }
        }

        buf += ", ";
    }
    buf.truncate(buf.len() - ", ".len());
    buf += ")";

    buf
}

/// Get an encounters group from the `abs_enemy_id` and `section_map`.
///
/// If `add_to_removed` is true, then any map with `(Old)` or `(Removed)` in its
/// name will be added to `removed_vec` rather than being added to the group.
/// This function is mainly a convenience so the logic doesn't have to appear
/// twice.
fn get_group<'a: 'b, 'b>(
    abs_enemy_id: u32,
    section_map: (SectionRef, Vec<&'a StageData<'a>>),
    removed_vec: &mut Vec<&'a StageData<'a>>,
    add_to_removed: bool,
) -> Group<'a> {
    let sec_ref = section_map.0;
    if *sec_ref.section().display_type() == DisplayType::Warn {
        log::warn!("{:?} stages encountered.", section_map.1[0].meta.type_enum);
    }

    let mut group = Group::new(sec_ref, vec![]);
    let group_chapters = &mut group.chapters;
    for stage in &section_map.1 {
        let stage_map = STAGE_WIKI_DATA
            .stage_map(stage.meta.type_num, stage.meta.map_num)
            .unwrap();

        if add_to_removed && OLD_OR_REMOVED_DETECT.is_match(&stage_map.name) {
            removed_vec.push(stage);
            continue;
        }
        // Add to removed and skip.
        if stage_map.name == "PLACEHOLDER" && stage_map.is_empty() {
            log::info!(
                "Map {:03}-{:03} is a placeholder.",
                stage.meta.type_num,
                stage.meta.map_num
            );
            continue;
        }
        // Remove placeholder maps. Technically doesn't need to happen since the
        // below statement will catch it without any errors, but it's a better
        // error message.

        let Some(stage_data) = stage_map.get(stage.meta.stage_num) else {
            log::info!(
                "Stage {:03}-{:03}-{:03} has no name.",
                stage.meta.type_num,
                stage.meta.map_num,
                stage.meta.stage_num
            );
            continue;
        };
        let stage_name = &stage_data.name;

        // If stage doesn't have a name in csv file, then skip.
        if !stage_name.starts_with('[') {
            log::info!("{stage_name:?} may be a placeholder. Skipping.",);
            continue;
        }
        // If stage name isn't a link, then skip.

        let map_name = OLD_OR_REMOVED_SUB.replace_all(&stage_map.name, "$1");
        // Get rid of `(Old)` and `(Removed)`.
        let chap = get_group_chapter(group_chapters, map_name);
        let mags = get_stage_mags(stage, abs_enemy_id);
        chap.stages.push(Stage::new(stage_name, mags, &stage.meta));
    }
    group
}

/// Collect sections map into encounter [Groups][Group].
fn get_encounter_groups<'a>(
    sections_map: Vec<(SectionRef, Vec<&'a StageData<'_>>)>,
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
        let group = get_group(abs_enemy_id, removed, &mut vec![], false);
        groups.push(group);
    }

    groups.sort_by(|s, o| s.sref.index().cmp(&o.sref.index()));

    groups
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
            if let Some(ids) = STAGE_WIKI_DATA.continue_id(encounter.meta.map_num) {
                let new_meta = match StageMeta::from_numbers(ids.0, ids.1, 999) {
                    Ok(nm) => nm,
                    Err(StageMetaParseError::Rejected) => {
                        assert_eq!(ids.0, 30);
                        StageMeta::from_selector_main(&ids.0.to_string(), &[999]).unwrap()
                    }
                    Err(StageMetaParseError::Invalid) => panic!("Matching meta failed or something."),
                };
                raw = raw_section(&new_meta);
            }
            // Use continuestages to get proper section.
            else {
                log::info!(
                    "Extra stage map {} has no continue id. Skipping.",
                    encounter.meta.map_num
                );
                continue;
            };
        }
        let raw = raw;

        if let Some(pos) = sections_map.iter().position(|(sref, _)| *sref == raw) {
            sections_map[pos].1.push(encounter);
        } else {
            sections_map.push((raw, vec![encounter]));
        };
        // if section in map then add to add to section, otherwise add new
        // section to map
    }
    sections_map
}

/// If enemy has always appeared at a certain mag, then remove mags after stage
/// names and replace with single message at top.
fn always_appeared_at(buf: &mut String) {
    let percentage_pattern = r" \([\d,%\s]+%\)\n";
    let re = Regex::new(percentage_pattern).unwrap();
    // This should probably be done in the actual code but oh well

    let map = re
        .find_iter(buf)
        .map(|cap| cap.as_str())
        .collect::<HashSet<&str>>();
    // set of all unique percentages
    if map.len() != 1 {
        return;
    }

    let diff_mags = r"\([\d,%\s]+% HP/[\d,%\s]+% AP\)\n";
    // instances of different hp and ap mags
    // FIXME this only works if enemy only appears once at different
    // magnfications, which isn't always true e.g. Satanmaria. Empirically it
    // probably doesn't matter though.
    // let diff_mags = r"\(([\d,%\s]+% HP/[\d,%\s]+% AP(, )?)+\)\n";
    let re = Regex::new(diff_mags).unwrap();

    let diff_map = re
        .find_iter(buf)
        .map(|cap| cap.as_str())
        .collect::<HashSet<&str>>();
    if !diff_map.is_empty() {
        return;
    }

    let mag = (*map.iter().next().unwrap()).to_string();
    if mag[1..].contains(' ') {
        // if is like "(10%, 100%)"
        return;
    }

    *buf = buf.replace(&mag, "\n");

    let matched: &(&str, [&str; 1]) = &Regex::new(r"\((.*)\)\n$")
        .unwrap()
        .captures(&mag)
        .unwrap()
        .extract();
    let percentage = matched.1[0];

    let repl = "{{Collapsible}}".to_string()
        + &format!("\nThis enemy has always appeared at {percentage} strength magnification.");
    *buf = buf.replace("{{Collapsible}}", &repl);
}

/// Post-process the buffer and apply some text transformations.
fn cleanup(buf: &mut String, abs_enemy_id: u32) {
    always_appeared_at(buf);
    manual_zoutbreak_replace(buf, abs_enemy_id);
}

/// Write the section text of an encounter group. Includes trailing newline.
fn write_encounter_group(buf: &mut String, group: Group<'_>) {
    if group.sref == SectionRef::EoC {
        *buf += "Strength magnifications are 100% in Chapter 1, 150% in \
                    Chapter 2, and 400% in Chapter 3.\n";
    }

    for mut chapter in group.chapters {
        if chapter.stages.is_empty() {
            log::info!("{:?} has no valid stages.", chapter.chapter_name);
            continue;
        }
        if chapter.chapter_name == "[[XP Stage]]" {
            continue;
        }
        if matches!(
            chapter.chapter_name,
            Cow::Borrowed("[[XP Stage|Weekend Stage]]")
        ) {
            // need to match against borrowed since otherwise old weekend
            // stage would also be matched
            chapter.chapter_name = Cow::Borrowed("[[XP Stage|XP Stage/Weekend Stage]]");
        }

        group.sref.section().fmt_chapter(buf, chapter.dedupped());
        *buf += "\n";
    }
}

/// temp
pub fn do_thing(wiki_id: u32, config: &Config) {
    let abs_enemy_id = wiki_id + 2;

    let all_stages = get_stages(config.version.current_version()).collect::<Vec<_>>();

    let mut encounters = all_stages
        .iter()
        .filter(|s| stage_contains_enemy(abs_enemy_id, s))
        .collect::<Vec<_>>();
    sort_encounters(&mut encounters);

    let section_map = get_section_map(&encounters);
    let groups = get_encounter_groups(section_map, abs_enemy_id);

    let mut buf = String::from("==Encounters==\n{{Collapsible}}");
    for group in groups {
        if group.chapters.is_empty() {
            continue;
        }

        write!(
            &mut buf,
            "\n==={heading}===\n",
            heading = group.sref.section().heading()
        )
        .unwrap();

        write_encounter_group(&mut buf, group);
    }
    buf += "</div>";

    cleanup(&mut buf, abs_enemy_id);

    println!("{buf}");

    /*
    ## extensions
    - [ ] analyse all stages to see if has same mag in all
    - [ ] analyse eoc outbreaks
    */
}

/*
Due to how the encounters section is constantly evolving, this module cannot be
tested any other way than empirically.

# Flow
## Wikitext
- Copy to clipboard, message saying "copied to clipboard" in green

Other things:
- Testing can be done easily for small parts but the overall thing can only be
  measured empirically
*/

/* Testing
- dedup
- always appeared at
  - Include with whitespace in only encounters
- Removed
- Extra
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_appeared_at() {
        let constant = "{{Collapsible}}
        ===[[Collaboration Events|Collaboration Stages]]===
        *This stage: this map (20%)

        ===[[:Category:Removed Content|Removed Stages]]===
        *This stage (Dessert): this map (20%)
        *This stage: this map (20%)
        </div>";
        let correct = "{{Collapsible}}\n\
        This enemy has always appeared at 20% strength magnification.
        ===[[Collaboration Events|Collaboration Stages]]===
        *This stage: this map

        ===[[:Category:Removed Content|Removed Stages]]===
        *This stage (Dessert): this map
        *This stage: this map
        </div>";

        let f = &mut constant.to_string();
        always_appeared_at(f);
        assert_eq!(f, correct);
    }
}
