#![allow(dead_code, unused_variables, missing_docs, unused_imports)]

use crate::data::cat::raw::{
    unitbuy::{self, UnitBuy},
    unitexp::Levelling,
};
use std::num::NonZero;
use strum::FromRepr;

#[repr(u8)]
#[derive(Debug, FromRepr)]
/// Currency used to unlock a unit.
enum UnlockCurrency {
    /// XP unlock.
    XP = 0,
    /// Catfood unlock.
    Catfood = 1,
    /// Unlock only available with capsules.
    None = 2,
}
#[derive(Debug)]
struct CatUnlock {
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

#[derive(Debug)]
struct EvolutionItem {
    item_id: u8,
    item_amt: u8,
}
#[derive(Debug)]
struct CatfruitEvolution {
    item_cost: [EvolutionItem; 5],
    xp_cost: u32,
    level_required: u8,
}
#[derive(Debug)]
enum EvolutionType {
    XP { level: u8 },
    Catfruit(CatfruitEvolution),
    Other,
}

#[derive(Debug)]
struct EvolutionInfo {
    evolution_id: NonZero<u32>,
    etype: EvolutionType,
}

#[derive(Debug)]
struct UpgradeCost {
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

#[derive(Debug)]
struct MaxLevels {
    ch1: u8,
    ch2: u8,
    initial_plus: u8,
    max_nat: u8,
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
#[derive(Debug, FromRepr)]
pub enum Rarity {
    Normal = 0,
    Special = 1,
    Rare = 2,
    SuperRare = 3,
    UberRare = 4,
    LegendRare = 5,
}

pub enum AncientEggInfo {
    None,
    Egg { normal: u8, evolved: u8 },
}

// egg data
// misc
pub struct Misc {
    rarity: Rarity,
    cro_order: i16,
    sell_xp: u32,
    sell_np: u8,
    update_released: i64,
    egg_info: AncientEggInfo,
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
            cro_order: unitbuy.cro_order,
            sell_xp: unitbuy.sell_xp,
            update_released: unitbuy.update_released,
            sell_np: unitbuy.sell_np,
            egg_info,
        }
    }
}

#[derive(Debug)]
struct UnitBuyData {
    unlock: CatUnlock,
    true_evol: Option<EvolutionInfo>,
    ultra_evol: Option<EvolutionInfo>,
    upgrade_costs: UpgradeCost,
    max_levels: MaxLevels,
}

// scale_type: Levelling,
// scale_type:Levelling::from_id(unitbuy)

impl UnitBuyData {
    fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
        let (true_evol, ultra_evol) = Self::get_evolutions(unitbuy);
        Self {
            unlock: CatUnlock::from_unitbuy(unitbuy),
            true_evol,
            ultra_evol,
            upgrade_costs: UpgradeCost::from_unitbuy(unitbuy),
            max_levels: MaxLevels::from_unitbuy(unitbuy),
        }
    }

    fn get_tf_evol(unitbuy: &UnitBuy) -> Option<EvolutionInfo> {
        let tf_num = NonZero::new(unitbuy.true_num)?;
        if unitbuy.evol_level > -1 {
            return Some(EvolutionInfo {
                evolution_id: tf_num,
                etype: EvolutionType::XP {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::TEST_CONFIG, data::cat::raw::unitbuy::UnitBuyContainer};

    #[test]
    fn test_units() {
        let version = TEST_CONFIG.version.current_version();
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();

        let test_units = [
            ("cat", 0),
            ("tank", 1),
            ("titan", 8),
            ("actress", 9),
            ("bahamut", 25),
            ("cancan", 32),
            ("dio", 177),
            ("metal", 200),
            ("dasli", 543),
            ("cat modoki", 626),
            ("sfeline", 643),
            ("courier", 658),
        ];
        for (name, id) in test_units {
            let unit = unitbuy.get_unit(id).unwrap();
            println!("{name} ({id}) = {:?}", unit);
            println!("{:?}\n", UnitBuyData::from_unitbuy(unit));
        }
    }
}
