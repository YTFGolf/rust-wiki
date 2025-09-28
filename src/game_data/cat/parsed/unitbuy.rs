//! Data from `unitbuy.csv`.

use crate::game_data::cat::raw::unitbuy::UnitBuyRaw;
use std::{fmt::Display, num::NonZero};
use strum::FromRepr;
mod tests;

#[repr(u8)]
#[derive(Debug, FromRepr, PartialEq, Eq, PartialOrd, Ord, Default)]
/// Currency used to unlock a unit.
pub enum UnlockCurrency {
    /// XP unlock.
    XP = 0,
    /// Catfood unlock.
    Catfood = 1,
    /// Unlock only available with capsules.
    #[default]
    None = 2,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
/// How the cat is unlocked.
pub struct CatUnlock {
    /// EoC stage the unit is available (cat is available before stage 0, tank
    /// before stage 1, bahamut 48).
    pub stage_available: u8,
    /// EoC chapter available (e.g. Bahamut = 2).
    pub chap_available: u8,
    /// Cost of unlocking the unit.
    pub unlock_cost: u16,
    /// Currency the unit is unlocked with.
    pub unlock_currency: UnlockCurrency,
}
impl CatUnlock {
    /// Create new [`CatUnlock`].
    pub fn new(
        stage_available: u8,
        chap_available: u8,
        unlock_cost: u16,
        unlock_currency: UnlockCurrency,
    ) -> CatUnlock {
        Self {
            stage_available,
            chap_available,
            unlock_cost,
            unlock_currency,
        }
    }

    fn from_unitbuy(unitbuy: &UnitBuyRaw) -> Self {
        Self {
            stage_available: unitbuy.stage_available,
            chap_available: unitbuy.chap_available,
            unlock_cost: unitbuy.unlock_cost,
            unlock_currency: UnlockCurrency::from_repr(unitbuy.unlock_currency).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Item used in Catfruit evolutions.
pub struct EvolutionItem {
    /// Item ID.
    pub item_id: u8,
    /// Amount of item.
    pub item_amt: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Descriptor for a unit's Catfruit evolution.
pub struct CatfruitEvolution {
    /// Catfruit/B.Stone cost.
    pub item_cost: [EvolutionItem; 5],
    /// XP Cost.
    pub xp_cost: u32,
    /// Level needed to evolve.
    pub level_required: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// How the unit might evolve into a specific form.
pub enum EvolutionType {
    /// Evolves by getting to level.
    Levels {
        /// Level that unit evolves at.
        level: u8,
        // could this just be a tuple?
    },
    /// Evolves by Catfruit.
    Catfruit(CatfruitEvolution),
    /// By drop.
    Other,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// How the unit might evolve.
pub struct EvolutionInfo {
    /// ID of evolution.
    ///
    /// E.g. an awakening stage will give a reward drop that corresponds to this
    /// number.
    pub evolution_id: NonZero<u32>,
    /// How the unit evolves into their new form.
    pub etype: EvolutionType,
}
impl EvolutionInfo {
    /// Get new [`EvolutionInfo`] without already needing a [`NonZero<u32>`].
    pub const fn new(evolution_id: u32, etype: EvolutionType) -> Self {
        Self {
            evolution_id: NonZero::new(evolution_id).unwrap(),
            etype,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Base cost of upgrading the unit each level.
///
/// This is not the full picture, this will also require usage of
/// `unitexp.csv`'s data to know the cost for each level.
pub struct UpgradeCost {
    /// Base costs from level 1 to level 10.
    pub costs: [u32; 10],
}

impl UpgradeCost {
    fn from_unitbuy(unitbuy: &UnitBuyRaw) -> Self {
        let costs = [
            unitbuy.upgrade_to_1,
            unitbuy.upgrade_to_2,
            unitbuy.upgrade_to_3,
            unitbuy.upgrade_to_4,
            unitbuy.upgrade_to_5,
            unitbuy.upgrade_to_6,
            unitbuy.upgrade_to_7,
            unitbuy.upgrade_to_8,
            unitbuy.upgrade_to_9,
            unitbuy.upgrade_to_10,
        ];
        Self { costs }
    }
}

/// Information from the page "Level-up".
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxLevels {
    /// Normal level cap.
    pub ch1: u8,
    /// Level cap after chap 2.
    pub ch2: u8,
    /// Initial plus level cap.
    pub initial_plus: u8,
    /// Includes Catseyes.
    pub max_nat: u8,
    /// Final level cap.
    pub max_plus: u8,
}
impl MaxLevels {
    fn from_unitbuy(unitbuy: &UnitBuyRaw) -> Self {
        Self {
            ch1: unitbuy.max_xp_level_ch1,
            ch2: unitbuy.max_xp_level_ch2,
            initial_plus: unitbuy.initial_max_plus,
            max_nat: unitbuy.max_nat_level,
            max_plus: unitbuy.max_plus_level,
        }
    }
}

#[repr(u8)]
#[derive(Debug, FromRepr, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Rarity of unit.
pub enum Rarity {
    /// Normal.
    Normal = 0,
    /// Special.
    Special = 1,
    /// Rare.
    Rare = 2,
    /// Super Rare.
    SuperRare = 3,
    /// Uber Rare.
    UberRare = 4,
    /// Legend Rare.
    LegendRare = 5,
}
impl Rarity {
    /// Get string representation of rarity.
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Normal => "Normal",
            Self::Special => "Special",
            Self::Rare => "Rare",
            Self::SuperRare => "Super Rare",
            Self::UberRare => "Uber Rare",
            Self::LegendRare => "Legend Rare",
        }
    }
}
impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Info about Ancient Eggs.
pub enum AncientEggInfo {
    /// Unit is not an egg.
    None,
    /// Unit is an egg.
    Egg {
        /// The "m" id of the normal form.
        ///
        /// E.g. if this is 0 then the deploy icon is "uni000_m00.png".
        normal: u8,
        /// "m" id of evolved form.
        ///
        /// E.g. if this is 2 then the deploy icon is "uni002_m01.png".
        evolved: u8,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Order of unit in the Cat Guide.
pub enum CatGuideOrder {
    /// Unit with id.
    Unit(u32),
    /// Unit is a summon and is not in the guide.
    Summon,
}
impl CatGuideOrder {
    fn is_summon(&self) -> bool {
        match self {
            CatGuideOrder::Unit(_) => false,
            CatGuideOrder::Summon => true,
        }
    }
}
impl From<i32> for CatGuideOrder {
    fn from(value: i32) -> Self {
        match value {
            x if x >= 0 => Self::Unit(x as u32),
            -1 => Self::Summon,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Miscellaneous unitbuy data.
pub struct Misc {
    /// Unit's rarity.
    pub rarity: Rarity,
    /// Order in Cat Guide.
    pub guide_order: CatGuideOrder,
    /// XP that unit sells for.
    pub sell_xp: u32,
    /// NP that unit sells for.
    pub sell_np: u8,
    /// Update unit was released (don't trust this tbh).
    pub update_released: i64,
    /// Ancinet Egg info.
    pub egg_info: AncientEggInfo,
}
impl Misc {
    fn from_unitbuy(unitbuy: &UnitBuyRaw) -> Self {
        let egg_info = match (unitbuy.ancient_egg_id_norm, unitbuy.ancient_egg_id_evo) {
            (-1, -1) => AncientEggInfo::None,
            (0, n) if n > 0 => AncientEggInfo::Egg {
                normal: 0,
                evolved: n as u8,
            },
            _ => unreachable!(),
        };
        Self {
            rarity: Rarity::from_repr(unitbuy.rarity).unwrap(),
            guide_order: unitbuy.cro_order.into(),
            sell_xp: unitbuy.sell_xp,
            update_released: unitbuy.update_released,
            sell_np: unitbuy.sell_np,
            egg_info,
        }
    }

    /// Is the unit a summon?
    pub fn is_summon(&self) -> bool {
        self.guide_order.is_summon()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// All data contained in `unitbuy.csv`.
pub struct UnitBuy {
    /// How to unlock the unit.
    pub unlock: CatUnlock,
    /// How to evolve unit into True Form.
    pub true_evol: Option<EvolutionInfo>,
    /// How to evolve unit into Ultra Form.
    pub ultra_evol: Option<EvolutionInfo>,
    /// Base upgrade costs for each level.
    pub upgrade_costs: UpgradeCost,
    /// Max level data for unit.
    pub max_levels: MaxLevels,
    /// Random data unitbuy also includes.
    pub misc: Misc,
}

impl UnitBuy {
    /// Get data from raw unitbuy data.
    pub fn from_unitbuy(unitbuy: &UnitBuyRaw) -> Self {
        let (true_evol, ultra_evol) = Self::get_evolutions(unitbuy);
        Self {
            unlock: CatUnlock::from_unitbuy(unitbuy),
            true_evol,
            ultra_evol,
            upgrade_costs: UpgradeCost::from_unitbuy(unitbuy),
            max_levels: MaxLevels::from_unitbuy(unitbuy),
            misc: Misc::from_unitbuy(unitbuy),
        }
    }

    fn get_tf_evol(unitbuy: &UnitBuyRaw) -> Option<EvolutionInfo> {
        let tf_num = NonZero::new(unitbuy.true_num)?;
        if unitbuy.evol_level > -1 {
            return Some(EvolutionInfo {
                evolution_id: tf_num,
                etype: EvolutionType::Levels {
                    level: unitbuy.evol_level as u8,
                },
            });
        }

        if unitbuy.true_cf_evol_level <= 0 {
            return Some(EvolutionInfo {
                evolution_id: tf_num,
                etype: EvolutionType::Other,
            });
        }

        let level_required = unitbuy.true_cf_evol_level as u8;
        let xp_cost = unitbuy.true_evol_xp;

        let item_cost = [
            EvolutionItem {
                item_id: unitbuy.true_cf_item1,
                item_amt: unitbuy.true_cf_cost1,
            },
            EvolutionItem {
                item_id: unitbuy.true_cf_item2,
                item_amt: unitbuy.true_cf_cost2,
            },
            EvolutionItem {
                item_id: unitbuy.true_cf_item3,
                item_amt: unitbuy.true_cf_cost3,
            },
            EvolutionItem {
                item_id: unitbuy.true_cf_item4,
                item_amt: unitbuy.true_cf_cost4,
            },
            EvolutionItem {
                item_id: unitbuy.true_cf_item5,
                item_amt: unitbuy.true_cf_cost5,
            },
        ];

        let etype = EvolutionType::Catfruit(CatfruitEvolution {
            item_cost,
            xp_cost,
            level_required,
        });

        Some(EvolutionInfo {
            evolution_id: tf_num,
            etype,
        })
    }

    fn get_uf_evol(unitbuy: &UnitBuyRaw) -> Option<EvolutionInfo> {
        let uf_num = NonZero::new(unitbuy.ultra_num)?;
        // if unitbuy.evol_level > -1 {
        //     return Some(EvolutionInfo {
        //         evolution_id: uf_num,
        //         etype: EvolutionType::XP {
        //             level: unitbuy.evol_level as u8,
        //         },
        //     });
        // }

        if unitbuy.ultra_cf_evol_level <= 0 {
            return Some(EvolutionInfo {
                evolution_id: uf_num,
                etype: EvolutionType::Other,
            });
        }

        let level_required = unitbuy.ultra_cf_evol_level as u8;
        let xp_cost = unitbuy.ultra_evol_xp;

        let item_cost = [
            EvolutionItem {
                item_id: unitbuy.ultra_cf_item1,
                item_amt: unitbuy.ultra_cf_cost1,
            },
            EvolutionItem {
                item_id: unitbuy.ultra_cf_item2,
                item_amt: unitbuy.ultra_cf_cost2,
            },
            EvolutionItem {
                item_id: unitbuy.ultra_cf_item3,
                item_amt: unitbuy.ultra_cf_cost3,
            },
            EvolutionItem {
                item_id: unitbuy.ultra_cf_item4,
                item_amt: unitbuy.ultra_cf_cost4,
            },
            EvolutionItem {
                item_id: unitbuy.ultra_cf_item5,
                item_amt: unitbuy.ultra_cf_cost5,
            },
        ];

        let etype = EvolutionType::Catfruit(CatfruitEvolution {
            item_cost,
            xp_cost,
            level_required,
        });

        Some(EvolutionInfo {
            evolution_id: uf_num,
            etype,
        })
    }
    fn get_evolutions(unitbuy: &UnitBuyRaw) -> (Option<EvolutionInfo>, Option<EvolutionInfo>) {
        let tf = Self::get_tf_evol(unitbuy);
        let uf = Self::get_uf_evol(unitbuy);
        (tf, uf)
    }
}
