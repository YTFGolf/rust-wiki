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

// from stage meta get heading
// removed is done just by string search
#[cfg(test)]
mod tests {
    use super::*;

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
        const name: &str = "[[Korea (Empire of Cats)|Korea]]";
        const mags: &str = "(100%)";

        let section = get_section_heading("[[Empire of Cats]]");
        assert_eq!(
            stringify(section, &korea, name, mags),
            "Stage 1: [[Korea (Empire of Cats)|Korea]]"
        );
    }
}
