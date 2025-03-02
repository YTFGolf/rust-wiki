use std::fmt::Write;

use super::types::{StageCodeType, StageType};
use crate::meta::stage::variant::{StageVariantID, VariantSize};

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

/// Clone item of [`RAW_STAGE_TYPES`] at compile time.
const fn clone(t: &StageType) -> StageType {
    let stage_code = match t.stage_code {
        C::Map => C::Map,
        C::RPrefix => C::RPrefix,
        C::Other(x) => C::Other(x),
        C::Custom => C::Custom,
    };

    init(
        t.name,
        t.map_code,
        stage_code,
        t.variant_id,
        t.common_name_match_str,
    )
}

const MAX_VARIANT_NUMBER: VariantSize = 37;
const MAX_VARIANT_INDEX: usize = MAX_VARIANT_NUMBER as usize + 1;
// store the data, store the map
const STAGE_TYPES: [Option<StageType>; MAX_VARIANT_INDEX] = {
    let mut a = [const { None }; MAX_VARIANT_INDEX];

    let mut i = 0;
    while i < RAW_STAGE_TYPES.len() {
        let raw = &RAW_STAGE_TYPES[i];
        let cloned = clone(raw);
        a[variant_to_index(raw.variant_id)] = Some(cloned);
        i += 1;
    }

    a
};

const fn variant_to_index(variant: StageVariantID) -> usize {
    variant.num() as usize
}

/// Get variant's stage type.
pub const fn get_stage_type(variant: StageVariantID) -> &'static StageType {
    let i = variant_to_index(variant);
    match &STAGE_TYPES[i] {
        Some(v) => v,
        None => panic!("Variant is not initialised properly!"),
    }
}

pub fn get_all_matchers() -> [String; RAW_STAGE_TYPES.len()] {
    fn a(st: &StageType) -> String {
        let mut a = st.common_name_match_str.to_string();
        if !a.is_empty() {
            a += "|"
        }
        write!(a, "{}", st.variant_id.num()).unwrap();
        if let Some(code) = st.map_code {
            write!(a, "|{code}").unwrap();
        }
        match st.stage_code {
            C::Map | C::Custom => (),
            C::RPrefix => write!(a, "|R{map}", map = st.map_code.unwrap()).unwrap(),
            C::Other(ex) => write!(a, "|{ex}").unwrap(),
        }
        a
    }

    RAW_STAGE_TYPES
        .iter()
        .map(a)
        .collect::<Vec<_>>()
        .try_into()
        .expect("Couldn't convert vec to array.")
}

// function to get the regex matching done properly.
// "z 1|z 2|z 3" e.g.
// All will get their map codes, stage codes and numbers added automatically
// if begins with z then there is a special case, maybe this could tell that

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
                .filter(|st| st.variant_id == variant)
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
        todo!("{:?}", get_all_matchers())
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
