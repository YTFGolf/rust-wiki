//! Deals with sections of encounters.

use crate::data::stage::raw::stage_metadata::{consts::StageTypeEnum as T, StageMeta};
use std::fmt::Write;

#[derive(Debug)]
/// How you display the section.
enum DisplayType {
    /// E.g. SoL: `*Stage x: name (mags)`.
    Story,
    /// Standard `*map: stage` or `*map:\n**stage 1`.
    Normal,
    /// `*{stage} {mags}`
    Flat,
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
impl EncountersSection {
    fn fmt_encounter_custom(buf: &mut String, meta: &StageMeta, name: &str, mags: &str) {
        // EoC
        if meta.type_enum == T::MainChapters && meta.map_num == 0 {
            if meta.stage_num <= 46 {
                write!(buf, "Stage {stage}: {name}", stage = meta.stage_num + 1).unwrap();
            } else {
                // can just use the chapter given in StageNames.csv
                let pos = name.len() - 2;
                let chap = &name[pos..];
                let name = &name[..pos];

                write!(buf, "Stage{chap}-48: {name}").unwrap();
            }

            return;
        }

        if meta.type_enum == T::MainChapters {
            write!(
                buf,
                "Stage {chap}-{stage}: {name} {mags}",
                chap = meta.map_num % 3 + 1,
                stage = meta.stage_num + 1,
                name = &name[..name.len() - " (N1)".len()]
            )
            .unwrap();
            return;
        }

        if meta.type_enum == T::Filibuster {
            write!(buf, "Stage 3-IN: {name} {mags}",).unwrap();
            return;
        }

        if meta.type_enum == T::AkuRealms {
            *buf += "Stage ";
            if meta.stage_num == 999 {
                *buf += "30-IN"
            } else {
                write!(buf, "{stage}", stage = meta.stage_num + 1).unwrap();
            }

            write!(buf, ": {name} {mags}").unwrap();

            return;
        }

        if meta.type_enum != T::Outbreaks {
            panic!("Type should be Outbreaks, not {:?}", meta.type_enum);
        }

        // TODO check if mags is empty before formatting
        // TODO something to do with the stage numbers and formatting if has
        // loads of stages in eoc outbreaks. probably works if map num is set to
        // 999.

        write!(
            buf,
            "Stage {chap}-{stage}: {name} {mags}",
            chap = meta.map_num + 1,
            stage = meta.stage_num + 1
        )
        .unwrap();
    }

    /// Write the non-asterisked part of an encounter.
    pub fn fmt_encounter(&self, buf: &mut String, meta: &StageMeta, stage_name: &str, mags: &str) {
        match self.display_type {
            D::Skip => (),
            D::Warn | D::Normal | D::Flat => {
                write!(buf, "{stage_name} {mags}").unwrap();
            }
            D::Story => {
                write!(
                    buf,
                    "Stage {chap}-{stage}: {stage_name} {mags}",
                    chap = meta.map_num + 1,
                    stage = meta.stage_num + 1
                )
                .unwrap();
            }
            D::Custom => Self::fmt_encounter_custom(buf, meta, stage_name, mags),
        }
    }
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

    get_new_section("[[Legend Stages#Stories of Legend|Stories of Legend]]", D::Story),
    get_new_section("[[Legend Stages#Uncanny Legends|Uncanny Legends]]",     D::Story),
    get_new_section("[[Legend Stages#Zero Legends|Zero Legends]]",           D::Story),

    get_new_section("[[Special Events|Event Stages]]",                       D::Normal),
    get_new_section("[[Underground Labyrinth]]",                             D::Flat),
    get_new_section("[[Collaboration Event Stages|Collaboration Stages]]",   D::Normal),
    get_new_section("[[Enigma Stages]]",                                     D::Normal),
    get_new_section("[[Catclaw Dojo]]",                                      D::Normal),

    get_new_section("Extra Stages",                                          D::Warn),
    get_new_section("[[Catamin Stages]]",                                    D::Skip),
];

// from stage meta get heading
// removed is done just by string search
#[cfg(test)]
mod tests {
    use super::*;
    use crate::wikitext::data_files::stage_page_data::STAGE_NAMES;

    /// Get an EncountersSection from its heading.
    fn get_section_heading(heading: &'static str) -> &EncountersSection {
        SECTIONS.iter().find(|s| s.heading == heading).unwrap()
    }

    fn stringify(
        section: &EncountersSection,
        meta: &StageMeta,
        stage_name: &str,
        mags: &str,
    ) -> String {
        let mut buf = String::from("");
        section.fmt_encounter(&mut buf, meta, stage_name, mags);
        buf
    }

    #[test]
    fn test_eoc_format() {
        let korea = StageMeta::new("eoc 0").unwrap();
        let name = &STAGE_NAMES.from_meta(&korea).unwrap().name;
        const MAGS: &str = "(100%)";

        let section = get_section_heading("[[Empire of Cats]]");
        assert_eq!(
            stringify(section, &korea, name, MAGS),
            "Stage 1: [[Korea (Empire of Cats)|Korea]]"
        );
    }

    #[test]
    fn test_eoc_moon() {
        let moon_ch2 = StageMeta::new("eoc 49").unwrap();
        let name = &STAGE_NAMES.from_meta(&moon_ch2).unwrap().name;
        const MAGS: &str = "(150%)";

        let section = get_section_heading("[[Empire of Cats]]");
        assert_eq!(
            stringify(section, &moon_ch2, name, MAGS),
            "Stage 2-48: [[Moon (Empire of Cats)|Moon]]"
        );
    }

    #[test]
    fn test_itf_format() {
        let great_abyss = StageMeta::new("itf 1 23").unwrap();
        let name = &STAGE_NAMES.from_meta(&great_abyss).unwrap().name;
        const MAGS: &str = "(150%)";

        let section = get_section_heading("[[Into the Future]]");
        assert_eq!(
            stringify(section, &great_abyss, name, MAGS),
            "Stage 1-24: [[The Great Abyss (Into the Future)|The Great Abyss]] (150%)"
        );
    }

    #[test]
    fn test_cotc_format() {
        let sighter_star = StageMeta::new("cotc 2 24").unwrap();
        let name = &STAGE_NAMES.from_meta(&sighter_star).unwrap().name;
        const MAGS: &str = "(150%)";

        let section = get_section_heading("[[Cats of the Cosmos]]");
        assert_eq!(
            stringify(section, &sighter_star, name, MAGS),
            "Stage 2-25: [[Sighter's Star (Cats of the Cosmos)|Sighter's Star]] (150%)"
        );
    }

    #[test]
    fn test_filibuster_format() {
        let mut filibuster = StageMeta::new("filibuster").unwrap();
        let name = &STAGE_NAMES.from_meta(&filibuster).unwrap().name;
        filibuster.map_num = 8;
        filibuster.stage_num = 999;
        // expected from ContinueStages

        const MAGS: &str = "(1,500%)";

        let section = get_section_heading("[[Cats of the Cosmos]]");
        assert_eq!(
            stringify(section, &filibuster, name, MAGS),
            "Stage 3-IN: [[Filibuster Invasion (Cats of the Cosmos)|Filibuster Invasion]] (1,500%)"
        );
    }

    #[test]
    fn test_aku_realms() {
        let korea = StageMeta::new("aku 0").unwrap();
        let name = &STAGE_NAMES.from_meta(&korea).unwrap().name;
        const MAGS: &str = "(100%)";

        let section = get_section_heading("[[The Aku Realms]]");
        assert_eq!(
            stringify(section, &korea, name, MAGS),
            "Stage 1: [[Korea (Aku Realm)|Korea]] (100%)"
        );
    }

    #[test]
    fn test_story_format() {
        let torture_room = StageMeta::new("sol 21 3").unwrap();
        let name = &STAGE_NAMES.from_meta(&torture_room).unwrap().name;
        const MAGS: &str = "(400%)";

        let section = get_section_heading("[[Legend Stages#Stories of Legend|Stories of Legend]]");
        assert_eq!(
            stringify(section, &torture_room, name, MAGS),
            "Stage 22-4: [[Torture Room]] (400%)"
        );
    }

    #[test]
    fn test_normal_format() {
        let xp_hard = StageMeta::new("event 28 2").unwrap();
        let name = &STAGE_NAMES.from_meta(&xp_hard).unwrap().name;
        const MAGS: &str = "(400%)";

        let section = get_section_heading("[[Special Events|Event Stages]]");
        assert_eq!(
            stringify(section, &xp_hard, name, MAGS),
            "[[Sweet XP (Hard)]] (400%)"
        );
    }

    #[test]
    fn always_appeared_at() {
        let xp_hard = StageMeta::new("event 28 2").unwrap();
        let name = &STAGE_NAMES.from_meta(&xp_hard).unwrap().name;
        const MAGS: &str = "";

        let section = get_section_heading("[[Special Events|Event Stages]]");
        assert_eq!(
            stringify(section, &xp_hard, name, MAGS),
            "[[Sweet XP (Hard)]]"
        );
    }

    // Test invasions for Face of God and Mount Aku

    // Encounter name filter or something
    // Remove all catamin stages
    // move removed to section
    // eliminate unlinked stages and warn
    // move extra stages into correct section
    // remove princess punt eoc stages
}
