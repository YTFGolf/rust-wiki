//! Types to deserialise map csv files.

// #[derive(Debug, serde::Deserialize)]
// #[allow(dead_code, missing_docs)]
// /// No real clue.
// // mapnum?,treasuredrop,scorerewards,?,?
// pub struct HeaderCSV {
//     map_file_num: i32,
// // "itemsetting" according to clamchowder
//     _unknown_1: i32,
// // Something to do with score rewards
//     _unknown_2: i32,
// // next 2 are something to do with unlock conditions
//     _unknown_3: i32,
//     _unknown_4: i32,

// 9,-1,-1,137000,137000
// Means map 9, and must complete map 37000 to unlock
// mapcondition and stagecondition are the 137000s according to clamchowder

// アイテム報酬型ステージ設定(-1:OFF),　スコア報酬型ステージ設定(-1:OFF)

// }

// not important probably
// #[derive(Debug, serde::Deserialize)]
// #[allow(dead_code, missing_docs)]
// pub struct Line2CSV {}

/*
 * Other things:
 * - stage_conditions.csv for Labyrinth
 * - stage_hint_popup.csv for tutorial
 */

#[derive(Debug, serde::Deserialize)]
/// All fixed data stored in the map file. Can reliably be deserialised
/// using serde.
pub struct StageInfoCSVFixed {
    /// Energy to challenge stage.
    ///
    /// If it is a Catamin stage then 0-999 is A, 1000-1999 is B, 2000-2999
    /// is C.
    pub energy: u32,
    /// Base XP rewarded.
    pub xp: u32,
    /// Music track played at beginning of stage.
    _init_track: u32,
    /// Base percentage where music changes to
    /// [_second_track][StageInfoCSVFixed::_second_track].
    _base_drop: u32,
    /// Music track played when base hp goes below
    /// [_base_drop][StageInfoCSVFixed::_base_drop].
    _second_track: u32,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// CSV data related to stage treasures.
pub struct TreasureCSV {
    /// Chance the item will drop.
    pub item_chance: u32,
    /// ID of item.
    pub item_id: u32,
    /// Amount of the item that drops.
    pub item_amt: u32,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
/// CSV data related to timed score rewards.
pub struct ScoreRewardsCSV {
    /// Score required to get item.
    pub score: u32,
    /// ID of item.
    pub item_id: u32,
    /// Amount of the item that drops.
    pub item_amt: u32,
}

#[derive(Debug, PartialEq)]
/// Treasure drop reward modifier.
///
/// All descriptions are purely speculative based on BCU code; if you have
/// access to the game you may want to actually check what is said here.
pub enum TreasureType {
    /// E.g. Merciless XP: first item is only available once. After that
    /// works exactly the same as
    /// [AllUnlimited][TreasureType::AllUnlimited].
    OnceThenUnlimited = 1,
    /// Default e.g. Catfruit Jubilee.
    ///
    /// E.g. if you have (50, 50, 50) as the chances then the effective
    /// chances are (50, 25, 12.5).
    AllUnlimited = 0,
    /// Appears to just be a single unlimited raw value. Difference between
    /// this and [AllUnlimited][TreasureType::AllUnlimited] is unclear.
    ///
    /// There are no occurrences of this value being used on stages with
    /// multiple treasure rewards as of 13.6.0.
    UnclearMaybeRaw = -1,
    /// Guaranteed item once e.g. any stage in Infernal Tower. Can't use a
    /// Treasure Radar to get any items.
    ///
    /// If has multiple items then each item's chance is (`item_chance` /
    /// total sum). The exact mechanism is unclear but this seems to be the
    /// case.
    GuaranteedOnce = -3,
    /// Same as [GuaranteedOnce][TreasureType::GuaranteedOnce] but with
    /// unlimited rewards.
    GuaranteedUnlimited = -4,
}

impl From<i32> for TreasureType {
    fn from(treasure_type: i32) -> Self {
        match treasure_type {
            1 => TreasureType::OnceThenUnlimited,
            0 => TreasureType::AllUnlimited,
            -1 => TreasureType::UnclearMaybeRaw,
            -3 => TreasureType::GuaranteedOnce,
            -4 => TreasureType::GuaranteedUnlimited,
            _ => panic!("{treasure_type} is not recognised!"),
        }
    }
}
// 1 = first item is once, rest are as in 0
// 0 = default: e.g. Catfruit Jubilee
// -1 = unclear, seems to be unlimited and raw percentages
// -3 = One of the following (1 time). Chances are `item_chance` / total
// -4 = No treasure radar, additive chances same as -3.

#[derive(Debug)]
/// Container struct for all of the data for an individual stage.
pub struct StageDataCSV {
    /// Data that is always fixed into the csv.
    pub fixed_data: StageInfoCSVFixed,
    /// Modifier for the treasure drop.
    pub treasure_type: TreasureType,
    /// Raw treasure drop data.
    pub treasure_drop: Vec<TreasureCSV>,
    /// Raw score rewards data.
    pub score_rewards: Vec<ScoreRewardsCSV>,
}
