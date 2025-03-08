//! Deals with sections of encounters.

use super::chapter::Chapter;
use crate::{
    meta::stage::{stage_id::StageID, variant::StageVariantID as T},
    wikitext::data_files::stage_wiki_data::STAGE_WIKI_DATA,
};
use std::fmt::Write;

#[derive(Debug, PartialEq)]
/// How you display the section.
pub enum DisplayType {
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

#[derive(Debug, PartialEq)]
/// Section of unit encounters.
///
/// Methods on this object are purely for formatting. Processes such as
/// filtering, determining what `mags` should be, and finding the names of
/// stages is assumed to already be done before any methods are called.
pub struct EncountersSection {
    heading: &'static str,
    display_type: DisplayType,
}
impl EncountersSection {
    /// Get display type.
    pub fn display_type(&self) -> &DisplayType {
        &self.display_type
    }

    /// Get heading.
    pub fn heading(&self) -> &'static str {
        self.heading
    }

    fn fmt_encounter_custom(buf: &mut String, id: &StageID, name: &str) {
        // EoC
        if id.variant() == T::MainChapters && id.map().num() == 0 {
            if id.num() <= 46 {
                write!(buf, "Stage {stage}: {name}", stage = id.num() + 1).unwrap();
            } else {
                // can just use the chapter given in StageNames.csv
                let pos = name.len() - 2;
                let chap = &name[pos..];
                let name = &name[..pos];

                write!(buf, "Stage{chap}-48: {name}").unwrap();
            }

            return;
        }

        if id.variant() == T::MainChapters {
            write!(
                buf,
                "Stage {chap}-{stage}: {name}",
                chap = id.map().num() % 3 + 1,
                stage = id.num() + 1,
                name = &name[..name.len() - " (N1)".len()]
            )
            .unwrap();
            return;
        }

        if id.variant() == T::Filibuster {
            write!(buf, "Stage 3-IN: {name}",).unwrap();
            return;
        }

        if id.variant() == T::AkuRealms {
            *buf += "Stage ";
            if id.num() == 999 {
                *buf += "30-IN";
            } else {
                write!(buf, "{stage}", stage = id.num() + 1).unwrap();
            }

            write!(buf, ": {name}").unwrap();

            return;
        }

        assert!(
            id.variant().is_outbreak(),
            "Type should be Outbreaks, not {:?}",
            id.variant()
        );

        write!(
            buf,
            "Stage {chap}-{stage}: {name}",
            chap = id.map().num() + 1,
            stage = id.num() + 1,
            name = &name[..name.len() - " (Z3)".len()]
        )
        .unwrap();
    }

    /// Write the non-asterisked part of an encounter.
    pub fn fmt_encounter(&self, buf: &mut String, id: &StageID, stage_name: &str, mags: &str) {
        match self.display_type {
            D::Skip => unreachable!(),
            D::Warn | D::Normal | D::Flat => {
                write!(buf, "{stage_name}").unwrap();
            }
            D::Story => {
                write!(buf, "Stage {chap}-", chap = id.map().num() + 1).unwrap();

                if id.num() == 999 {
                    *buf += "IN";
                } else {
                    write!(buf, "{stage}", stage = id.num() + 1).unwrap();
                }

                write!(buf, ": {stage_name}").unwrap();
            }
            D::Custom => Self::fmt_encounter_custom(buf, id, stage_name),
        }

        if !(mags.is_empty()) {
            *buf += " ";
            *buf += mags;
        }
    }

    /// Write a chapter of encounters.
    pub fn fmt_chapter(&self, buf: &mut String, chapter: Chapter) {
        assert!(!chapter.stages.is_empty());
        match self.display_type {
            D::Skip => unreachable!(),
            D::Normal | D::Warn => {
                if chapter.stages.len() == 1 {
                    write!(buf, "*{chap}: ", chap = chapter.chapter_name).unwrap();
                    let stage = &chapter.stages[0];
                    self.fmt_encounter(buf, &stage.meta.into(), stage.stage_name, &stage.mags);

                    return;
                }

                write!(buf, "*{chap}:", chap = chapter.chapter_name).unwrap();
                for stage in chapter.stages {
                    *buf += "\n**";
                    self.fmt_encounter(buf, &stage.meta.into(), stage.stage_name, &stage.mags);
                }
            }
            D::Story | D::Flat | D::Custom => {
                // Custom is being done like this since it's only main chaps at
                // the moment
                for stage in chapter.stages {
                    *buf += "*";

                    let stage_id: StageID = stage.meta.into();
                    let stage_id = match stage_id.variant() {
                        T::Extra => {
                            if let Some(ids) = STAGE_WIKI_DATA.continue_id(stage_id.map().num()) {
                                StageID::from_numbers(ids.0, ids.1, 999)
                            } else {
                                todo!()
                            }
                        }
                        _ => stage_id,
                    };
                    // Get correct numbers for continue stages.

                    self.fmt_encounter(buf, &stage_id, stage.stage_name, &stage.mags);
                    *buf += "\n";
                }
                buf.pop();
            }
        }
    }
}

const fn get_new_section(heading: &'static str, display_type: DisplayType) -> EncountersSection {
    EncountersSection {
        heading,
        display_type,
    }
}

#[rustfmt::skip]
/// Available sections.
// Don't update without updating SectionRef and the first test.
const SECTIONS: [EncountersSection; 18] = [
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
    get_new_section("[[Collaboration Events|Collaboration Stages]]",         D::Normal),
    get_new_section("[[Enigma Stages]]",                                     D::Normal),
    get_new_section("[[Catclaw Dojo]]",                                      D::Normal),
    get_new_section("[[:Category:Removed Content|Removed Stages]]",          D::Normal),

    get_new_section("Extra Stages",                                          D::Warn),
    get_new_section("[[Catamin Stages]]",                                    D::Skip),
];

const _: () = assert!(std::mem::size_of::<SectionRef>() == std::mem::size_of::<SectionRefRepr>());
type SectionRefRepr = u8;
// make sure that this stays in line with the representation of SectionRef

#[repr(u8)]
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
/// Enum reference to a section.
pub enum SectionRef {
    EoC,
    EocOutbreak,
    ItF,
    ItfOutbreak,
    CotC,
    CotcOutbreak,
    AkuRealms,
    //
    SoL,
    UL,
    ZL,
    //
    Event,
    Labyrinth,
    Collab,
    Enigma,
    Dojo,
    Removed,
    //
    Extra,
    Catamin,
}
impl SectionRef {
    /// Get the index of the section in the ordered list of sections. Can be
    /// used as an ordering function.
    pub const fn index(&self) -> SectionRefRepr {
        unsafe { *std::ptr::from_ref::<SectionRef>(self).cast::<SectionRefRepr>() }
        // Casts the borrow to a SectionRef pointer (obviously borrows are
        // pointers with extra compiler magic), then converts that to a pointer
        // to a SectionRefRepr pointer, which can then be dereferenced without
        // the borrow checker complaining.

        // Safety: as long as SectionRefRepr is kept in line with SectionRef's
        // memory representation this works fine. I.e. if they take up the same
        // number of bytes then no information is lost or corrupted when doing
        // raw casts. This behaviour is guaranteed at compile time with the
        // const assert above.

        // Unsafe is necessary, otherwise calling this function on a borrowed
        // SectionRef would require a clone, which is just completely
        // unnecessary when the function can take care of that detail itself.
    }
    /// Get the defined section.
    pub const fn section(&self) -> &'static EncountersSection {
        &SECTIONS[self.index() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        meta::stage::stage_types::parse::parse_stage::parse_stage_selector,
        wikitext::{data_files::stage_wiki_data::STAGE_WIKI_DATA, encounters::chapter::Stage},
    };
    use std::borrow::Cow;
    use SectionRef as Ref;

    #[test]
    fn assert_section_ref() {
        assert_eq!(Ref::EoC.section().heading, "[[Empire of Cats]]");
        assert_eq!(
            Ref::EocOutbreak.section().heading,
            "[[Empire of Cats]] [[Zombie Outbreaks|Outbreaks]]"
        );
        assert_eq!(Ref::ItF.section().heading, "[[Into the Future]]");
        assert_eq!(
            Ref::ItfOutbreak.section().heading,
            "[[Into the Future]] [[Zombie Outbreaks|Outbreaks]]"
        );
        assert_eq!(Ref::CotC.section().heading, "[[Cats of the Cosmos]]");
        assert_eq!(
            Ref::CotcOutbreak.section().heading,
            "[[Cats of the Cosmos]] [[Zombie Outbreaks|Outbreaks]]"
        );
        assert_eq!(Ref::AkuRealms.section().heading, "[[The Aku Realms]]");
        assert_eq!(
            Ref::SoL.section().heading,
            "[[Legend Stages#Stories of Legend|Stories of Legend]]"
        );
        assert_eq!(
            Ref::UL.section().heading,
            "[[Legend Stages#Uncanny Legends|Uncanny Legends]]"
        );
        assert_eq!(
            Ref::ZL.section().heading,
            "[[Legend Stages#Zero Legends|Zero Legends]]"
        );
        assert_eq!(
            Ref::Event.section().heading,
            "[[Special Events|Event Stages]]"
        );
        assert_eq!(
            Ref::Labyrinth.section().heading,
            "[[Underground Labyrinth]]"
        );
        assert_eq!(
            Ref::Collab.section().heading,
            "[[Collaboration Events|Collaboration Stages]]"
        );
        assert_eq!(Ref::Enigma.section().heading, "[[Enigma Stages]]");
        assert_eq!(Ref::Dojo.section().heading, "[[Catclaw Dojo]]");
        assert_eq!(
            Ref::Removed.section().heading,
            "[[:Category:Removed Content|Removed Stages]]"
        );
        assert_eq!(Ref::Extra.section().heading, "Extra Stages");
        assert_eq!(Ref::Catamin.section().heading, "[[Catamin Stages]]");
    }

    fn stringify(
        section: &EncountersSection,
        id: &StageID,
        stage_name: &str,
        mags: &str,
    ) -> String {
        let mut buf = String::new();
        section.fmt_encounter(&mut buf, id, stage_name, mags);
        buf
    }

    #[test]
    fn single_eoc_format() {
        let korea = parse_stage_selector("eoc 0").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&korea).unwrap().name;
        const MAGS: &str = "";

        let section = Ref::EoC.section();
        assert_eq!(
            stringify(section, &korea, name, MAGS),
            "Stage 1: [[Korea (Empire of Cats)|Korea]]"
        );
    }

    #[test]
    fn single_eoc_moon() {
        let moon_ch2 = parse_stage_selector("eoc 49").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&moon_ch2).unwrap().name;
        const MAGS: &str = "";

        let section = Ref::EoC.section();
        assert_eq!(
            stringify(section, &moon_ch2, name, MAGS),
            "Stage 2-48: [[Moon (Empire of Cats)|Moon]]"
        );
    }

    #[test]
    fn single_itf_format() {
        let great_abyss = parse_stage_selector("itf 1 23").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&great_abyss).unwrap().name;
        const MAGS: &str = "(150%)";

        let section = Ref::ItF.section();
        assert_eq!(
            stringify(section, &great_abyss, name, MAGS),
            "Stage 1-24: [[The Great Abyss (Into the Future)|The Great Abyss]] (150%)"
        );
    }

    #[test]
    fn single_cotc_format() {
        let sighter_star = parse_stage_selector("cotc 2 24").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&sighter_star).unwrap().name;
        const MAGS: &str = "(150%)";

        let section = Ref::CotC.section();
        assert_eq!(
            stringify(section, &sighter_star, name, MAGS),
            "Stage 2-25: [[Sighter's Star (Cats of the Cosmos)|Sighter's Star]] (150%)"
        );
    }

    #[test]
    fn single_filibuster_format() {
        let mut filibuster = parse_stage_selector("filibuster").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&filibuster).unwrap().name;
        filibuster.set_map(8);
        filibuster.set_num(999);
        // expected from ContinueStages

        const MAGS: &str = "(1,500%)";

        let section = Ref::CotC.section();
        assert_eq!(
            stringify(section, &filibuster, name, MAGS),
            "Stage 3-IN: [[Filibuster Invasion (Cats of the Cosmos)|Filibuster Invasion]] (1,500%)"
        );
    }

    #[test]
    fn single_aku_realms() {
        let korea = parse_stage_selector("aku 0").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&korea).unwrap().name;
        const MAGS: &str = "(100%)";

        let section = Ref::AkuRealms.section();
        assert_eq!(
            stringify(section, &korea, name, MAGS),
            "Stage 1: [[Korea (Aku Realm)|Korea]] (100%)"
        );
    }

    #[test]
    fn single_story_format() {
        let torture_room = parse_stage_selector("sol 21 3").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&torture_room).unwrap().name;
        const MAGS: &str = "(400%)";

        let section = Ref::SoL.section();
        assert_eq!(
            stringify(section, &torture_room, name, MAGS),
            "Stage 22-4: [[Torture Room]] (400%)"
        );
    }

    #[test]
    fn single_normal_format() {
        let xp_hard = parse_stage_selector("event 28 2").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&xp_hard).unwrap().name;
        const MAGS: &str = "(400%)";

        let section = Ref::Event.section();
        assert_eq!(
            stringify(section, &xp_hard, name, MAGS),
            "[[Sweet XP (Hard)]] (400%)"
        );
    }

    #[test]
    fn single_z_outbreak() {
        let zoutbreak = parse_stage_selector("eocz 2 43").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&zoutbreak).unwrap().name;
        const MAGS: &str = "(600%)";

        let section = Ref::AkuRealms.section();
        assert_eq!(
            stringify(section, &zoutbreak, name, MAGS),
            "Stage 3-44: [[Las Vegas (Empire of Cats)|Las Vegas]] (600%)"
        );
    }

    #[test]
    fn single_aku_invasion() {
        let name = &STAGE_WIKI_DATA
            .stage(&StageID::from_numbers(4, 42, 0))
            .unwrap()
            .name;
        let mount_aku_repr = parse_stage_selector("aku 999").unwrap();

        const MAGS: &str = "(400%)";

        let section = Ref::AkuRealms.section();
        assert_eq!(
            stringify(section, &mount_aku_repr, name, MAGS),
            "Stage 30-IN: [[Mount Aku (Aku Realm)/Invasion|Mount Aku Invasion]] (400%)"
        );
    }

    #[test]
    fn single_doron_invasion() {
        let name = &STAGE_WIKI_DATA
            .stage(&StageID::from_numbers(4, 68, 0))
            .unwrap()
            .name;
        let idi_invasion_repr = parse_stage_selector("sol 35 999").unwrap();

        const MAGS: &str = "(400%)";

        let section = Ref::SoL.section();
        assert_eq!(
            stringify(section, &idi_invasion_repr, name, MAGS),
            "Stage 36-IN: [[The Face of God/Invasion|The Face of God Invasion]] (400%)"
        );
    }

    #[test]
    fn single_always_appeared_at() {
        let xp_hard = parse_stage_selector("event 28 2").unwrap();
        let name = &STAGE_WIKI_DATA.stage(&xp_hard).unwrap().name;
        const MAGS: &str = "";

        let section = Ref::Event.section();
        assert_eq!(
            stringify(section, &xp_hard, name, MAGS),
            "[[Sweet XP (Hard)]]"
        );
    }

    #[test]
    fn chapter_normal() {
        use crate::data::stage::raw::stage_metadata::LegacyStageMeta;

        let mut buf = String::new();
        let section = Ref::Event.section();
        section.fmt_chapter(
            &mut buf,
            Chapter::new(
                Cow::Borrowed("Chapter"),
                vec![
                    Stage::new(
                        "Stage 1",
                        "(100%)".to_string(),
                        &LegacyStageMeta::new("event 0 0").unwrap(),
                    ),
                    Stage::new(
                        "Stage 2",
                        String::new(),
                        &LegacyStageMeta::new("event 0 1").unwrap(),
                    ),
                    Stage::new(
                        "Stage 3",
                        "(1,500% HP/2% AP)".to_string(),
                        &LegacyStageMeta::new("event 0 2").unwrap(),
                    ),
                ],
            ),
        );

        assert_eq!(
            buf,
            "*Chapter:\n\
            **Stage 1 (100%)\n\
            **Stage 2\n\
            **Stage 3 (1,500% HP/2% AP)"
        );
    }

    #[test]
    fn chapter_flat() {
        use crate::data::stage::raw::stage_metadata::LegacyStageMeta;

        let mut buf = String::new();
        let section = Ref::Labyrinth.section();
        section.fmt_chapter(
            &mut buf,
            Chapter::new(
                Cow::Borrowed("Chapter"),
                vec![
                    Stage::new(
                        "Stage 1",
                        "(100%)".to_string(),
                        &LegacyStageMeta::new("l 0 0").unwrap(),
                    ),
                    Stage::new(
                        "Stage 2",
                        String::new(),
                        &LegacyStageMeta::new("l 0 1").unwrap(),
                    ),
                    Stage::new(
                        "Stage 3",
                        "(1,500% HP/2% AP)".to_string(),
                        &LegacyStageMeta::new("l 0 2").unwrap(),
                    ),
                ],
            ),
        );

        assert_eq!(
            buf,
            "*Stage 1 (100%)\n\
            *Stage 2\n\
            *Stage 3 (1,500% HP/2% AP)"
        );
    }

    #[test]
    fn chapter_story() {
        use crate::data::stage::raw::stage_metadata::LegacyStageMeta;

        let mut buf = String::new();
        let section = Ref::SoL.section();
        section.fmt_chapter(
            &mut buf,
            Chapter::new(
                Cow::Borrowed("Chapter"),
                vec![
                    Stage::new(
                        "Stage 1",
                        "(100%)".to_string(),
                        &LegacyStageMeta::new("sol 0 0").unwrap(),
                    ),
                    Stage::new(
                        "Stage 2",
                        String::new(),
                        &LegacyStageMeta::new("sol 0 1").unwrap(),
                    ),
                    Stage::new(
                        "Stage 3",
                        "(1,500% HP/2% AP)".to_string(),
                        &LegacyStageMeta::new("sol 0 2").unwrap(),
                    ),
                ],
            ),
        );

        assert_eq!(
            buf,
            "*Stage 1-1: Stage 1 (100%)\n\
            *Stage 1-2: Stage 2\n\
            *Stage 1-3: Stage 3 (1,500% HP/2% AP)"
        );
    }
}
