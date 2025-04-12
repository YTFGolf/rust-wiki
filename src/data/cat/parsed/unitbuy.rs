#![allow(dead_code, unused_variables, missing_docs, unused_imports)]

use std::num::NonZero;
use crate::data::cat::raw::unitbuy::{self, UnitBuy};
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

// evolutions
// upgrade_cost
// egg data
// misc

#[derive(Debug)]
struct Temp {
    unlock: CatUnlock,
    true_evol: Option<EvolutionInfo>,
    ultra_evol: Option<EvolutionInfo>,
}

impl Temp {
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
        if unitbuy.evol_level > -1 {
            return Some(EvolutionInfo {
                evolution_id: uf_num,
                etype: EvolutionType::XP {
                    level: unitbuy.evol_level as u8,
                },
            });
        }

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

    fn from_unitbuy(unitbuy: &UnitBuy) -> Self {
        let (true_evol, ultra_evol) = Self::get_evolutions(unitbuy);
        Self {
            unlock: CatUnlock::from_unitbuy(unitbuy),
            true_evol,
            ultra_evol,
        }
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
            let unit = unitbuy.get_unit(id);
            println!("{name} ({id}) = {:?}", unit);
            println!("{:?}\n", Temp::from_unitbuy(unit));
        }
    }
}

// {
//     pub upgrade_to_1: u32,
//     pub upgrade_to_2: u32,
//     pub upgrade_to_3: u32,
//     pub upgrade_to_4: u32,
//     pub upgrade_to_5: u32,
//     pub upgrade_to_6: u32,
//     pub upgrade_to_7: u32,
//     pub upgrade_to_8: u32,

//     // 10
//     pub upgrade_to_9: u32,
//     pub upgrade_to_10: u32,
//     pub rarity: u8,
//     pub cro_order: i16,
//     pub sell_xp: u32,
//     _uk17: u8,
//     pub max_xp_level_ch2: u8,
//     pub initial_max_plus: u8,

//     // 20
//     _uk21: u8,
//     // 2 for iron wall, 10 for everyone else
//     pub max_xp_level_ch1: u8,

//     _uk49: i8,
//     // -1 for normal cats, 30 for every cat that can go to 30. 31 for iron wall,
//     // 21 for Metal, 2 for units with max level 1. Perhaps first catseye level?

//     // 50
//     pub max_nat_level: u8,
//     pub max_plus_level: u8,
//     _uk52: u8,
//     _uk53: u16,
//     _uk54: u16,
//     _uk55: u8,
//     _uk56: u8,
//     // is 0 for normals and metal cat, 2 for everyone else
//     pub update_released: String,
//     // e.g. "90500" for 09.05.00 = 9.5.0
//     pub sell_np: u8,
//     _uk59: u32,

//     // 60
//     _uk60: u8,
//     // is 1 if cat is superfeline
//     pub ancient_egg_id_norm: i8,
//     pub ancient_egg_id_evo: i8,

//     #[serde(default)]
//     pub rest: Vec<i32>,
// }
