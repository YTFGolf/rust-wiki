use super::types::{StageCodeType, StageType};
use crate::meta::stage::variant::{StageVariantID, VariantSize};
use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

type T = StageVariantID;
type C = StageCodeType;

/// Initialise raw stage type.
const fn init(
    name: &'static str,
    map_code: Option<&'static str>,
    stage_code: StageCodeType,
    variant_id: StageVariantID,
    common_name_match_str: &'static str,
) -> StageType {
    StageType {
        name,
        map_code,
        stage_code,
        variant_id,
        common_name_match_str,
    }
}

// raw data for stage types
#[rustfmt::skip]
const RAW_STAGE_TYPES: [StageType; 24] = [
    init("Stories of Legend",            Some("N"),  C::RPrefix,     T::SoL,            "SoL"),
    init("Event Stages",                 Some("S"),  C::RPrefix,     T::Event,          "Event|Special"),
    init("Collaboration Stages",         Some("C"),  C::RPrefix,     T::Collab,         "Collab"),
    init("Main Chapters",                None,       C::Custom,      T::MainChapters,   "main|EoC|ItF|W|CotC|Space"),
    init("Extra Stages",                 Some("RE"), C::Other("EX"), T::Extra,          "Extra"),
    init("Catclaw Dojo",                 Some("T"),  C::RPrefix,     T::Dojo,           "Dojo"),
    init("Towers",                       Some("V"),  C::RPrefix,     T::Tower,          "Tower"),
    init("Ranking Dojo",                 Some("R"),  C::RPrefix,     T::RankingDojo,    "Rank"),
    init("Challenge Battle",             Some("M"),  C::RPrefix,     T::Challenge,      "Challenge"),

    init("Uncanny Legends",              Some("NA"), C::RPrefix,     T::UL,             "UL"),
    init("Catamin Stages",               Some("B"),  C::RPrefix,     T::Catamin,        "Catamin"),
    // init("Legend Quest",                 Some("D"),  C::Map,         T::LegendQuest,    "Haha"),
    init("Empire of Cats Outbreaks",     None,       C::Custom,      T::EocOutbreak,    "Z 1|Z 2|Z 3"),
    init("Into the Future Outbreaks",    None,       C::Custom,      T::ItfOutbreak,    "Z 4|Z 5|Z 6"),
    init("Cats of the Cosmos Outbreaks", None,       C::Custom,      T::CotcOutbreak,   "Z 7|Z 8|Z 9"),
    init("Filibuster Invasion",          None,       C::Custom,      T::Filibuster,     "Filibuster"),
    init("Gauntlets",                    Some("A"),  C::RPrefix,     T::Gauntlet,       "Gauntlet|Baron"),
    init("Enigma Stages",                Some("H"),  C::RPrefix,     T::Enigma,         "Enigma"),
    init("Collab Gauntlets",             Some("CA"), C::RPrefix,     T::CollabGauntlet, ""),

    init("Aku Realms",                   Some("DM"), C::Map,         T::AkuRealms,      "Aku"),
    init("Behemoth Culling",             Some("Q"),  C::RPrefix,     T::Behemoth,       "Behemoth"),
    init("Labyrinth",                    Some("L"),  C::Map,         T::Labyrinth,      "Labyrinth"),
    init("Zero Legends",                 Some("ND"), C::RPrefix,     T::ZL,             "ZL"),
    init("Colosseum",                    Some("SR"), C::RPrefix,     T::Colosseum,      "Colosseum"),
    init("Catclaw Championships",        Some("G"),  C::Map,         T::Championships,  "Championships"),
];

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct DataMatcher {
    arr: Vec<String>,
    re: Regex,
}
#[derive(Debug)]
/// Monolith struct for internal use.
pub struct StageTypeDataContainer {
    data: &'static StageType,
    matcher: DataMatcher,
}

const MAX_VARIANT_NUMBER: VariantSize = 37;
const MAX_VARIANT_INDEX: usize = MAX_VARIANT_NUMBER as usize + 1;
type StageTypesConstType = [Option<StageTypeDataContainer>; MAX_VARIANT_INDEX];

const fn variant_to_index(variant: StageVariantID) -> usize {
    variant.num() as usize
}

fn get_stage_types() -> StageTypesConstType {
    let mut a = [const { None }; MAX_VARIANT_INDEX];

    for raw in RAW_STAGE_TYPES.iter() {
        let cont = StageTypeDataContainer {
            data: raw,
            matcher: get_data_matcher(raw),
        };
        a[variant_to_index(raw.variant_id)] = Some(cont);
    }

    a
}

fn get_data_matcher(stype: &StageType) -> DataMatcher {
    let mut arr: Vec<String> = stype
        .common_name_match_str
        .split('|')
        .filter_map(|p| {
            if p.is_empty() {
                None
            } else {
                Some(p.to_string())
            }
        })
        .collect();

    arr.push(stype.variant_id.num().to_string());

    if let Some(code) = stype.map_code {
        arr.push(code.to_string());
    }
    match stype.stage_code {
        C::Map | C::Custom => (),
        C::RPrefix => arr.push(format!("R{map}", map = stype.map_code.unwrap())),
        C::Other(ex) => arr.push(ex.to_string()),
    }

    let pattern = format!("^({pattern})$", pattern = arr.join("|"));
    let re = RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .unwrap();

    DataMatcher { arr, re }
}

// -----------------------------------------------------------------------------

static STAGE_TYPES: LazyLock<StageTypesConstType> = LazyLock::new(get_stage_types);

/// Get variant's stage type.
pub fn get_stage_type(variant: StageVariantID) -> &'static StageTypeDataContainer {
    let i = variant_to_index(variant);
    STAGE_TYPES[i]
        .as_ref()
        .unwrap_or_else(|| panic!("Variant {variant:?} is not initialised properly!"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_variants() {
        // assert that array has all variants and numbers don't exceed
        // [`MAX_VARIANT_NUMBER`].
        for variant in StageVariantID::iter() {
            let is_in_map = STAGE_TYPES
                .iter()
                .flatten()
                .filter(|st| st.data.variant_id == variant)
                .collect::<Vec<_>>();
            let len = is_in_map.len();
            assert_eq!(len, 1, "{variant:?} should appear exactly once.");

            assert!(
                variant.num() <= MAX_VARIANT_NUMBER,
                "Variant {variant:?} has a value higher than {MAX_VARIANT_NUMBER}."
            );
            // this is probably unnecessary due to how STAGE_TYPES is
            // calculated, as that would report an error at compile time
        }
    }

    #[test]
    fn test_has_valid_map_and_stage_codes() {
        todo!()
        // check that all functions return a value (except custom on identifier)
    }

    #[test]
    fn assert_matchers_are_unique() {
        let _ = &STAGE_TYPES[0];
        todo!("{STAGE_TYPES:#?}")
        // check that no duplicate match possibilities exist
    }

    // assert that none always matches with custom
    // assert all others work the conventional way
    // assert no duplicates appear in raw types
    // assert all codes are upper case
}

/*
Plan:
- Does the equivalent of STAGE_TYPES: contains data about stages.
- Below applies to two separate sibling modules. All tests from LegacyStageMeta
  also get moved over to here. Siblings can use file names to implement a
  temporary from parser for LegacyStageMeta.
- This or a sibling module deals with parsing selectors. This or a sibling
  module deals with turning this information into real-world data (e.g. file
  names like `MapStageDataA_000.csv`).
*/

// impl StageType {
//     /// Code used in map files.
//     fn map_code(&self) -> Option<&'static str> {
//         self.map_code
//     }

//     /// Code used in stage files.
//     fn stage_code(&self) -> &StageCodeType {
//         &self.stage_code
//     }

//     // Get identifier (map,rprefix=map,other=other,custom=unimplemented)
//     // fn stage_ident(&self) -> &'static str {
//     //     todo!()
//     // }
// }

// function to get the regex matching done properly.
// "z 1|z 2|z 3" e.g.
// All will get their map codes, stage codes and numbers added automatically
// if begins with z then there is a special case, maybe this could tell that
