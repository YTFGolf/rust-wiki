#![allow(dead_code, unused_variables, missing_docs, unused_imports)]

use crate::data::cat::raw::{
    unitbuy::{self, UnitBuy},
    unitexp::Levelling,
};
use std::num::NonZero;
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
pub struct CatUnlock {
    /// EoC stage the unit is available (cat is available before stage 0, tank
    /// before stage 1, bahamut 48).
    stage_available: u8,
    /// EoC chapter available (e.g. Bahamut = 2).
    chap_available: u8,
    /// Cost of unlocking the unit.
    unlock_cost: u16,
    /// Currency the unit is unlocked with.
    unlock_currency: UnlockCurrency,
}
impl CatUnlock {
    fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
        Self {
            stage_available: unitbuy.stage_available,
            chap_available: unitbuy.chap_available,
            unlock_cost: unitbuy.unlock_cost,
            unlock_currency: UnlockCurrency::from_repr(unitbuy.unlock_currency).unwrap(),
        }
    }
}
impl CatUnlock {
    fn new(
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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EvolutionItem {
    item_id: u8,
    item_amt: u8,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CatfruitEvolution {
    item_cost: [EvolutionItem; 5],
    xp_cost: u32,
    level_required: u8,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum EvolutionType {
    /// Evolves by getting to level.
    Levels { level: u8 },
    /// Evolves by Catfruit.
    Catfruit(CatfruitEvolution),
    /// By drop.
    Other,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EvolutionInfo {
    evolution_id: NonZero<u32>,
    etype: EvolutionType,
}
impl EvolutionInfo {
    fn new(evolution_id: u32, etype: EvolutionType) -> Self {
        Self {
            evolution_id: NonZero::new(evolution_id).unwrap(),
            etype,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UpgradeCost {
    costs: [u32; 10],
}

impl UpgradeCost {
    fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
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
    ch1: u8,
    /// Level cap after chap 2.
    ch2: u8,
    /// Initial plus level cap.
    initial_plus: u8,
    /// Includes Catseyes.
    max_nat: u8,
    /// Final level cap.
    max_plus: u8,
}
impl MaxLevels {
    fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
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
#[derive(Debug, FromRepr, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rarity {
    Normal = 0,
    Special = 1,
    Rare = 2,
    SuperRare = 3,
    UberRare = 4,
    LegendRare = 5,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AncientEggInfo {
    None,
    Egg { normal: u8, evolved: u8 },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CatGuideOrder {
    Unit(u32),
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
pub struct Misc {
    pub rarity: Rarity,
    pub guide_order: CatGuideOrder,
    pub sell_xp: u32,
    pub sell_np: u8,
    pub update_released: i64,
    pub egg_info: AncientEggInfo,
}
impl Misc {
    fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
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

    pub fn is_summon(&self) -> bool {
        self.guide_order.is_summon()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitBuyData {
    pub unlock: CatUnlock,
    pub true_evol: Option<EvolutionInfo>,
    pub ultra_evol: Option<EvolutionInfo>,
    pub upgrade_costs: UpgradeCost,
    pub max_levels: MaxLevels,
    pub misc: Misc,
}

impl UnitBuyData {
    pub fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
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

    fn get_tf_evol(unitbuy: &UnitBuy) -> Option<EvolutionInfo> {
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

    fn get_uf_evol(unitbuy: &UnitBuy) -> Option<EvolutionInfo> {
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
    fn get_evolutions(unitbuy: &UnitBuy) -> (Option<EvolutionInfo>, Option<EvolutionInfo>) {
        let tf = Self::get_tf_evol(unitbuy);
        let uf = Self::get_uf_evol(unitbuy);
        (tf, uf)
    }
}
