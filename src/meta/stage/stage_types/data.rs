//! Contains data about all different stage types.

use super::types::{StageCodeType, StageType};
use crate::meta::stage::variant::{StageVariantID, VariantSize};
use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;

/// Type.
type T = StageVariantID;
/// Code.
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
        variant_id,
        name,
        map_code,
        stage_code,
        common_name_match_str,
    }
}

pub const SELECTOR_SEPARATOR: char = ' ';
#[rustfmt::skip]
/// Stage types container.
const RAW_STAGE_TYPES: [StageType; 24] = [
    // Matcher is possible common names for the stage type, separated by a pipe
    // character.
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
    init("Empire of Cats Outbreaks",     None,       C::Custom,      T::EocOutbreak,    "eocZ"),
    init("Into the Future Outbreaks",    None,       C::Custom,      T::ItfOutbreak,    "itfZ"),
    init("Cats of the Cosmos Outbreaks", None,       C::Custom,      T::CotcOutbreak,   "cotcZ"),
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
/// Struct containing a [`Regex`] matcher of possible aliases for the
/// [`StageType`].
pub struct DataMatcher {
    /// List of all possible aliases.
    pub arr: Vec<String>,
    /// Case-insensitive regex matcher of these aliases.
    pub re: Regex,
}
#[derive(Debug)]
/// Monolith of data about a stage type.
pub struct StageTypeDataContainer {
    /// [`StageType`] information about the stage type.
    pub data: &'static StageType,
    /// Regex matcher information about the stage type.
    pub matcher: DataMatcher,
}

/// Max numeric value of any variant.
const MAX_VARIANT_NUMBER: VariantSize = 37;
/// [`MAX_VARIANT_NUMBER`], usable as an array index.
const MAX_VARIANT_INDEX: usize = MAX_VARIANT_NUMBER as usize + 1;
/// Type for the inner data of [`STAGE_TYPES`].
type StageTypesConstType = [Option<StageTypeDataContainer>; MAX_VARIANT_INDEX];

/// Convert stage variant ID to [`STAGE_TYPES`] index.
const fn variant_to_index(variant: StageVariantID) -> usize {
    variant.num() as usize
}

/// Get [`None`]-padded array of relevant stage type data.
fn get_stage_types() -> StageTypesConstType {
    let mut a = [const { None }; MAX_VARIANT_INDEX];

    for raw in &RAW_STAGE_TYPES {
        let cont = StageTypeDataContainer {
            data: raw,
            matcher: get_data_matcher(raw),
        };
        a[variant_to_index(raw.variant_id)] = Some(cont);
    }

    a
}

/// Get the [`StageType`]'s appropriate [`DataMatcher`].
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

/// Container for all stage types.
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
    use regex::escape;
    use std::{collections::HashSet, iter::zip};
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
        }
    }

    #[test]
    // REQUIRED to ensure that following functions have correct behaviour.
    fn assert_iter_correct() {
        // assert that properly `iter`ed STAGE_TYPES is same as RAW_STAGE_TYPES

        // allows other tests here to fearlessly iterate through raw types
        // also in doing so asserts that no duplicates appear in raw types and
        // all variants are in raw exactly once, although this is only due to
        // architecture

        let mut counter = 0;
        // make sure that it is definitely going through each element of both
        for (parsed, raw) in zip(STAGE_TYPES.iter().flatten(), RAW_STAGE_TYPES) {
            counter += 1;
            assert_eq!(
                parsed.data, &raw,
                "Mismatch between parsed and raw stage types statics. \
                Did you order raw incorrectly?",
            );
        }

        assert_eq!(counter, RAW_STAGE_TYPES.len());
        assert_eq!(
            counter,
            STAGE_TYPES.iter().flatten().collect::<Vec<_>>().len()
        );
    }

    #[test]
    fn has_valid_map_and_stage_codes() {
        for raw in RAW_STAGE_TYPES {
            // assert that map: None always matches with stage: Custom
            if raw.map_code.is_none() || raw.stage_code == C::Custom {
                assert!(
                    raw.map_code.is_none() && raw.stage_code == C::Custom,
                    "{t:?} has undefined behaviour with stage and map codes.",
                    t = raw.variant_id
                );
            }
        }
    }

    #[test]
    fn assert_matchers_are_unique_and_valid() {
        // check that no duplicate match possibilities exist and each matcher
        // doesn't contain invalid characters
        // invalid character check is necessary because it makes other functions
        // so much simpler
        let mut seen = HashSet::new();
        for cont in STAGE_TYPES.iter().flatten() {
            for pattern in &cont.matcher.arr {
                assert!(
                    !pattern.contains(SELECTOR_SEPARATOR),
                    "Stage type pattern {pat:?} in variant {var:?} \
                    contains invalid character {c:?}",
                    pat = pattern,
                    var = cont.data.variant_id,
                    c = SELECTOR_SEPARATOR
                );

                assert!(
                    seen.insert(pattern),
                    "Duplicated matcher pattern {pattern:?} found for stage type {:?}",
                    cont.data.variant_id
                );
            }
        }
    }

    #[test]
    fn assert_upper_codes() {
        // this is based on current data, it's entirely possible this test may
        // need to be removed in future
        // assert all map codes (and ex stage code) are upper case
        fn is_upper(s: &str) -> bool {
            s.chars().all(|c| c.is_ascii_uppercase())
        }

        for raw in RAW_STAGE_TYPES {
            if let Some(s) = raw.map_code {
                assert!(is_upper(s));
            }

            match raw.stage_code {
                C::Other(s) => assert!(is_upper(s)),
                C::Map | C::RPrefix | C::Custom => (),
            }
        }
    }

    #[test]
    fn test_escaped_matchers() {
        // ensure matchers do not need to be escaped
        // This is only done so that I don't need to escape matchers at runtime
        // If this absolutely needs to be changed, then change the
        // initialisation of matcher and remove this test
        for cont in STAGE_TYPES.iter().flatten() {
            for pattern in &cont.matcher.arr {
                assert_eq!(
                    pattern,
                    &escape(pattern),
                    "Pattern on {:?} contains special Regex characters.",
                    cont.data.variant_id
                );
            }
        }
    }
}
