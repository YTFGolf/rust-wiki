#![allow(dead_code, unused_variables, missing_docs, unused_imports)]

use crate::data::cat::raw::{
    unitbuy::{self, UnitBuy},
    unitexp::Levelling,
};
use std::num::NonZero;
use strum::FromRepr;

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
    Levels { level: u8 },
    Catfruit(CatfruitEvolution),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaxLevels {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::TEST_CONFIG,
        data::{cat::raw::unitbuy::UnitBuyContainer, version::Version},
    };

    #[test]
    #[ignore]
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
        todo!()
    }

    fn get_unitbuy(id: u32, version: &Version) -> UnitBuyData {
        let unitbuy = version.get_cached_file::<UnitBuyContainer>();
        let unit = unitbuy.get_unit(id).unwrap();
        UnitBuyData::from_unitbuy(unit)
    }

    const NORMAL_MAX: MaxLevels = MaxLevels {
        ch1: 10,
        ch2: 20,
        initial_plus: 19,
        max_nat: 20,
        max_plus: 90,
    };

    #[test]
    fn basic() {
        let version = TEST_CONFIG.version.current_version();

        let cat = get_unitbuy(0, version);

        let unlock = CatUnlock::new(0, 0, 0, UnlockCurrency::XP);
        let true_evol = EvolutionInfo::new(19001, EvolutionType::Levels { level: 30 });
        let costs = [
            200, 400, 700, 1_100, 1_600, 2_200, 2_900, 3_700, 4_600, 5_600,
        ];
        let misc = Misc {
            rarity: Rarity::Normal,
            guide_order: CatGuideOrder::Unit(0),
            sell_xp: 10_000,
            sell_np: 1,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            cat,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: None,
                upgrade_costs: UpgradeCost { costs },
                max_levels: NORMAL_MAX,
                misc
            }
        )
    }

    #[test]
    fn basic2() {
        let version = TEST_CONFIG.version.en();
        // jp has initial cost set at 500

        let tank = get_unitbuy(1, version);

        let unlock = CatUnlock::new(1, 0, 0, UnlockCurrency::XP);
        let true_evol = EvolutionInfo::new(19002, EvolutionType::Levels { level: 30 });
        let costs = [
            400, 700, 1_600, 2_800, 4_300, 6_100, 8_200, 10_600, 13_300, 16_300,
        ];
        let misc = Misc {
            rarity: Rarity::Normal,
            guide_order: CatGuideOrder::Unit(1),
            sell_xp: 12_000,
            sell_np: 1,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            tank,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: None,
                upgrade_costs: UpgradeCost { costs },
                max_levels: NORMAL_MAX,
                misc
            }
        )
    }

    #[test]
    fn basic3() {
        let version = TEST_CONFIG.version.current_version();

        let titan = get_unitbuy(8, version);

        let unlock = CatUnlock::new(24, 0, 8_000, UnlockCurrency::XP);
        let true_evol = EvolutionInfo::new(19009, EvolutionType::Levels { level: 30 });
        let costs = [
            2_000, 3_500, 6_200, 9_800, 14_300, 19_700, 26_000, 33_200, 41_300, 50_300,
        ];
        let misc = Misc {
            rarity: Rarity::Normal,
            guide_order: CatGuideOrder::Unit(80),
            sell_xp: 24_000,
            sell_np: 1,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            titan,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: None,
                upgrade_costs: UpgradeCost { costs },
                max_levels: NORMAL_MAX,
                misc
            }
        )
    }

    const SPECIAL_MAX: MaxLevels = MaxLevels {
        ch1: 10,
        ch2: 20,
        initial_plus: 0,
        max_nat: 50,
        max_plus: 0,
    };

    #[test]
    fn special() {
        const EVOL_ID: u32 = 10010;
        const GUIDE: u32 = 1000090;
        const SELL_XP: u32 = 999;

        let version = TEST_CONFIG.version.current_version();

        let actress = get_unitbuy(9, version);

        let unlock = CatUnlock::new(7, 1, 150, UnlockCurrency::Catfood);
        let true_evol = EvolutionInfo::new(EVOL_ID, EvolutionType::Other);
        let costs = [
            800, 1_600, 3_100, 5_100, 7_600, 10_600, 14_100, 18_100, 22_600, 27_600,
        ];
        let misc = Misc {
            rarity: Rarity::Special,
            guide_order: CatGuideOrder::Unit(GUIDE),
            sell_xp: SELL_XP,
            sell_np: 1,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            actress,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: None,
                upgrade_costs: UpgradeCost { costs },
                max_levels: SPECIAL_MAX,
                misc
            }
        )
    }

    #[test]
    fn bahamut() {
        const EVOL_ID: u32 = 18026;
        const GUIDE: u32 = 1910010;
        const SELL_XP: u32 = 999;

        let version = TEST_CONFIG.version.current_version();

        let bahamut = get_unitbuy(25, version);

        let unlock = CatUnlock::new(48, 2, 0, UnlockCurrency::XP);
        let true_evol = EvolutionInfo::new(EVOL_ID, EvolutionType::Other);
        let costs = [
            8_000, 10_000, 16_000, 24_000, 34_000, 46_000, 60_000, 76_000, 94_000, 114_000,
        ];
        let misc = Misc {
            rarity: Rarity::Special,
            guide_order: CatGuideOrder::Unit(GUIDE),
            sell_xp: SELL_XP,
            sell_np: 1,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            bahamut,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: None,
                upgrade_costs: UpgradeCost { costs },
                max_levels: SPECIAL_MAX,
                misc
            }
        )
    }

    const SR_MAX: MaxLevels = MaxLevels {
        ch1: 10,
        ch2: 20,
        initial_plus: 9,
        max_nat: 50,
        max_plus: 70,
    };
    const UR_MAX: MaxLevels = MaxLevels {
        max_nat: 60,
        ..SR_MAX
    };

    enum EvolutionItemVariant {
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
    type I = EvolutionItemVariant;

    const fn evol_item(id: EvolutionItemVariant, amt: u8) -> EvolutionItem {
        EvolutionItem {
            item_id: id as u8,
            item_amt: amt,
        }
    }

    #[test]
    fn cancan() {
        const EVOL_ID: u32 = 15033;
        const GUIDE: u32 = 3000000;
        const SELL_XP: u32 = 478000;
        const SELL_NP: u8 = 15;

        let version = TEST_CONFIG.version.current_version();

        let cancan = get_unitbuy(32, version);

        let unlock = CatUnlock::default();

        let tf_costs = [
            evol_item(I::PurpleFruit, 2),
            evol_item(I::RedFruit, 4),
            evol_item(I::BlueFruit, 1),
            evol_item(I::YellowFruit, 1),
            evol_item(I::EpicFruit, 1),
        ];
        let true_evol = EvolutionInfo::new(
            EVOL_ID,
            EvolutionType::Catfruit(CatfruitEvolution {
                item_cost: tf_costs,
                xp_cost: 500_000,
                level_required: 30,
            }),
        );

        let costs = [
            6_250, 8_200, 12_400, 17_800, 24_800, 42_400, 64_500, 93_000, 148_000, 298_000,
        ];
        let misc = Misc {
            rarity: Rarity::SuperRare,
            guide_order: CatGuideOrder::Unit(GUIDE),
            sell_xp: SELL_XP,
            sell_np: SELL_NP,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            cancan,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: None,
                upgrade_costs: UpgradeCost { costs },
                max_levels: SR_MAX,
                misc
            }
        )
    }

    #[test]
    fn dio() {
        const TRUE_ID: u32 = 15178;
        const ULTRA_ID: u32 = 25178;
        const GUIDE: u32 = 4003050;
        const SELL_XP: u32 = 1000000;
        const SELL_NP: u8 = 50;

        let version = TEST_CONFIG.version.current_version();

        let dio = get_unitbuy(177, version);

        let unlock = CatUnlock::default();

        let tf_costs = [
            evol_item(I::GreenFruit, 5),
            evol_item(I::PurpleFruit, 4),
            evol_item(I::BlueFruit, 3),
            evol_item(I::YellowFruit, 7),
            evol_item(I::EpicFruit, 4),
        ];
        let true_evol = EvolutionInfo::new(
            TRUE_ID,
            EvolutionType::Catfruit(CatfruitEvolution {
                item_cost: tf_costs,
                xp_cost: 1_000_000,
                level_required: 30,
            }),
        );

        let uf_costs = [
            evol_item(I::YellowStone, 2),
            evol_item(I::BlueStone, 5),
            evol_item(I::YellowFruit, 5),
            evol_item(I::BlueFruit, 5),
            evol_item(I::Nothing, 0),
        ];
        let ultra_evol = EvolutionInfo::new(
            ULTRA_ID,
            EvolutionType::Catfruit(CatfruitEvolution {
                item_cost: uf_costs,
                xp_cost: 2_000_000,
                level_required: 60,
            }),
        );

        let costs = [
            7_800, 9_800, 14_800, 21_800, 42_500, 64_300, 93_200, 118_000, 197_400, 513_500,
        ];
        let misc = Misc {
            rarity: Rarity::UberRare,
            guide_order: CatGuideOrder::Unit(GUIDE),
            sell_xp: SELL_XP,
            sell_np: SELL_NP,
            update_released: 0,
            egg_info: AncientEggInfo::None,
        };

        assert_eq!(
            dio,
            UnitBuyData {
                unlock,
                true_evol: Some(true_evol),
                ultra_evol: Some(ultra_evol),
                upgrade_costs: UpgradeCost { costs },
                max_levels: UR_MAX,
                misc
            }
        )
    }
    // TODO this should realistically be moved to another module
}
