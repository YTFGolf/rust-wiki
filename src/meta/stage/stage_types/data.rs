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
/*
// note that names may be changed and need to update whole file first

also fun fact legend quest has prefix D
*/

use super::types::{StageCodeType, StageType};
use crate::meta::stage::variant::StageVariantID;

type T = StageVariantID;
type C = StageCodeType;

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

const MAX_RAW_TYPES: usize = 1;
#[rustfmt::skip]
const RAW_STAGE_TYPES: [StageType; MAX_RAW_TYPES] = [
    init("Stories of Legend", Some("N"), C::RPrefix, T::SoL, "SoL"),
];
// pub const LEGACY_STAGE_TYPES: [LegacyStageType; 22] = [
//     initialise_stage_type("Stories of Legend",            000, "N",     true,  T::SoL),
//     initialise_stage_type("Event Stages",                 001, "S",     true,  T::Event),
//     initialise_stage_type("Collaboration Stages",         002, "C",     true,  T::Collab),
//     initialise_stage_type("Main Chapters",                003, "main",  false, T::MainChapters),
//     initialise_stage_type("Extra Stages",                 004, "RE|EX", false, T::Extra),
//     initialise_stage_type("Catclaw Dojo",                 006, "T",     true,  T::Dojo),
//     initialise_stage_type("Towers",                       007, "V",     true,  T::Tower),
//     initialise_stage_type("Ranking Dojo",                 011, "R",     true,  T::RankingDojo),
//     initialise_stage_type("Challenge Battle",             012, "M",     true,  T::Challenge),

//     initialise_stage_type("Uncanny Legends",              013, "NA",    true,  T::UL),
//     initialise_stage_type("Catamin Stages",               014, "B",     true,  T::Catamin),
//     // initialise_stage_type("Empire of Cats Outbreaks",     020, "",      false, T::EocOutbreak),
//     // initialise_stage_type("Into the Future Outbreaks",    021, "",      false, T::ItfOutbreak),
//     // initialise_stage_type("Cats of the Cosmos Outbreaks", 022, "",      false, T::CotcOutbreak),
//     initialise_stage_type("Outbreaks",                    999, "main",  false, T::Outbreaks),

//     initialise_stage_type("Filibuster Invasion",          023, "",      false, T::Filibuster),
//     initialise_stage_type("Gauntlets",                    024, "A",     true,  T::Gauntlet),
//     initialise_stage_type("Enigma Stages",                025, "H",     true,  T::Enigma),
//     initialise_stage_type("Collab Gauntlets",             027, "CA",    true,  T::CollabGauntlet),

//     initialise_stage_type("Aku Realms",                   030, "DM",    false, T::AkuRealms),
//     initialise_stage_type("Behemoth Culling",             031, "Q",     true,  T::Behemoth),
//     initialise_stage_type("Labyrinth",                    033, "L",     false, T::Labyrinth),
//     initialise_stage_type("Zero Legends",                 034, "ND",    true,  T::ZL),
//     initialise_stage_type("Colosseum",                    036, "SR",    true,  T::Colosseum),
//     initialise_stage_type("Catclaw Championships",        037, "G",     false, T::Championships),
// ];

// static STAGE_TYPE_MAP: LazyLock<[LegacyStageTypeMap; 22]> = LazyLock::new(|| {[
//     initialise_type_map("SoL|0|N|RN",               T::SoL),
//     initialise_type_map("Event|Special|1|S|RS",     T::Event),
//     initialise_type_map("Collab|2|C|RC",            T::Collab),
//     initialise_type_map("EoC|ItF|W|CotC|Space",     T::MainChapters),
//     initialise_type_map("Extra|4|RE|EX",            T::Extra),
//     initialise_type_map("Dojo|6|T|RT",              T::Dojo),
//     initialise_type_map("Tower|7|V|RV",             T::Tower),
//     initialise_type_map("Rank|11|R|RR",             T::RankingDojo),
//     initialise_type_map("Challenge|12|M|RM",        T::Challenge),

//     initialise_type_map("UL|13|NA|RNA",             T::UL),
//     initialise_type_map("Catamin|14|B|RB",          T::Catamin),
//     initialise_type_map("Z",                        T::Outbreaks),
//     initialise_type_map("Filibuster|23",            T::Filibuster),
//     // initialise_type_map("LQ|16|D",                  "Why would you want to do Legend Quest"),
//     initialise_type_map("Gauntlet|Baron|24|A|RA",   T::Gauntlet),
//     initialise_type_map("Enigma|25|H|RH",           T::Enigma),
//     initialise_type_map("27|CA|RCA",                T::CollabGauntlet),
//     initialise_type_map("Aku|30|DM",                T::AkuRealms),

//     initialise_type_map("Behemoth|31|Q|RQ",         T::Behemoth),
//     initialise_type_map("Labyrinth|33|L",           T::Labyrinth),
//     initialise_type_map("ZL|34|ND|RND",             T::ZL),
//     initialise_type_map("Colosseum|36|SR|RSR",      T::Colosseum),
//     initialise_type_map("Championships|37|G",       T::Championships),
// ]});

const MAX_VARIANT_NUMBER: usize = 37;
// store the data, store the map
const STAGE_TYPES: [Option<StageType>; MAX_VARIANT_NUMBER] = [const { None }; MAX_VARIANT_NUMBER];

const fn variant_to_index(variant: StageVariantID) -> usize {
    variant.num() as usize
}

/// Get variant's stage type.
pub fn get_stage_type(variant: StageVariantID) -> &'static StageType {
    let i = variant_to_index(variant);
    match &STAGE_TYPES[i] {
        Some(v) => v,
        None => panic!("Variant is not initialised properly!"),
    }
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

            let variant_num = usize::try_from(variant.num())
                .expect("Error when converting from stage variant number to usize.");
            assert!(
                variant_num <= MAX_VARIANT_NUMBER,
                "Variant {variant:?} has a value higher than {MAX_VARIANT_NUMBER}."
            );
        }
    }

    #[test]
    fn test_has_valid_map_and_stage_codes() {
        todo!()
        // check that all functions return a value (except custom on identifier)
    }

    #[test]
    fn assert_matchers_are_unique() {
        todo!()
        // check that no duplicate match possibilities exist
    }
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
