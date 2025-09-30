//! Convenient enum for evolution items.

use strum::FromRepr;

/// Shorthand to give names to item ids.
#[derive(Debug, PartialEq, Eq, Copy, Clone, FromRepr)]
#[allow(missing_docs)]
pub enum EvolutionItemVariant {
    Nothing = 0,
    PurpleSeed = 30,
    RedSeed = 31,
    BlueSeed = 32,
    GreenSeed = 33,
    YellowSeed = 34,
    PurpleFruit = 35,
    RedFruit = 36,
    BlueFruit = 37,
    GreenFruit = 38,
    YellowFruit = 39,
    EpicFruit = 40,
    ElderSeed = 41,
    ElderFruit = 42,
    EpicSeed = 43,
    GoldFruit = 44,
    PurpleStone = 167,
    RedStone = 168,
    BlueStone = 169,
    GreenStone = 170,
    YellowStone = 171,
    PurpleGem = 179,
    RedGem = 180,
    BlueGem = 181,
    GreenGem = 182,
    YellowGem = 183,
    EpicStone = 184,
}
