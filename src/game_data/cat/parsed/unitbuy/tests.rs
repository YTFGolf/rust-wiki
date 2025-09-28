#![cfg(test)]

use super::*;
use crate::{
    TEST_CONFIG,
    game_data::{cat::raw::unitbuy::UnitBuyContainer, version::Version},
};
use strum::IntoEnumIterator;

fn get_unitbuy(id: u32, version: &Version) -> UnitBuy {
    let unitbuy = version.get_cached_file::<UnitBuyContainer>();
    let unit = unitbuy.get_unit(id).unwrap();
    UnitBuy::from_unitbuy(unit)
}

const NORMAL_MAX: MaxLevels = MaxLevels {
    ch1: 10,
    ch2: 20,
    initial_plus: 19,
    max_nat: 20,
    max_plus: 90,
};

const SPECIAL_MAX: MaxLevels = MaxLevels {
    ch1: 10,
    ch2: 20,
    initial_plus: 0,
    max_nat: 50,
    max_plus: 0,
};

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

const UR_COSTS: [u32; 10] = [
    7_800, 9_800, 14_800, 21_800, 42_500, 64_300, 93_200, 118_000, 197_400, 513_500,
];

#[allow(dead_code)]
/// Shorthand to give names to item ids.
// TODO this should be somewhere else and only imported from here.
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
fn basic() {
    let version = TEST_CONFIG.version.current_version();

    let cat = get_unitbuy(0, version);
    let guide_cmp_const = cat.misc.guide_order.clone();

    let unlock = CatUnlock::new(0, 0, 0, UnlockCurrency::XP);
    let true_evol = EvolutionInfo::new(19001, EvolutionType::Levels { level: 30 });
    let costs = [
        200, 400, 700, 1_100, 1_600, 2_200, 2_900, 3_700, 4_600, 5_600,
    ];
    let misc = Misc {
        rarity: Rarity::Normal,
        guide_order: guide_cmp_const,
        sell_xp: 10_000,
        sell_np: 1,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        cat,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: NORMAL_MAX,
            misc
        }
    );
}

#[test]
fn basic2() {
    let version = TEST_CONFIG.version.en();
    // jp has initial cost set at 500

    let tank = get_unitbuy(1, version);
    let guide_cmp_const = tank.misc.guide_order.clone();

    let unlock = CatUnlock::new(1, 0, 0, UnlockCurrency::XP);
    let true_evol = EvolutionInfo::new(19002, EvolutionType::Levels { level: 30 });
    let costs = [
        400, 700, 1_600, 2_800, 4_300, 6_100, 8_200, 10_600, 13_300, 16_300,
    ];
    let misc = Misc {
        rarity: Rarity::Normal,
        guide_order: guide_cmp_const,
        sell_xp: 12_000,
        sell_np: 1,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        tank,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: NORMAL_MAX,
            misc
        }
    );
}

#[test]
fn basic3() {
    let version = TEST_CONFIG.version.current_version();

    let titan = get_unitbuy(8, version);
    let guide_cmp_const = titan.misc.guide_order.clone();

    let unlock = CatUnlock::new(24, 0, 8_000, UnlockCurrency::XP);
    let true_evol = EvolutionInfo::new(19009, EvolutionType::Levels { level: 30 });
    let costs = [
        2_000, 3_500, 6_200, 9_800, 14_300, 19_700, 26_000, 33_200, 41_300, 50_300,
    ];
    let misc = Misc {
        rarity: Rarity::Normal,
        guide_order: guide_cmp_const,
        sell_xp: 24_000,
        sell_np: 1,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        titan,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: NORMAL_MAX,
            misc
        }
    );
}

#[test]
fn special() {
    const EVOL_ID: u32 = 10010;
    const SELL_XP: u32 = 999;

    let version = TEST_CONFIG.version.current_version();

    let actress = get_unitbuy(9, version);
    let guide_cmp_const = actress.misc.guide_order.clone();

    let unlock = CatUnlock::new(7, 1, 150, UnlockCurrency::Catfood);
    let true_evol = EvolutionInfo::new(EVOL_ID, EvolutionType::Other);
    let costs = [
        800, 1_600, 3_100, 5_100, 7_600, 10_600, 14_100, 18_100, 22_600, 27_600,
    ];
    let misc = Misc {
        rarity: Rarity::Special,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: 1,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        actress,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: SPECIAL_MAX,
            misc
        }
    );
}

#[test]
fn bahamut() {
    const EVOL_ID: u32 = 18026;
    const SELL_XP: u32 = 999;

    let version = TEST_CONFIG.version.current_version();

    let bahamut = get_unitbuy(25, version);
    let guide_cmp_const = bahamut.misc.guide_order.clone();

    let unlock = CatUnlock::new(48, 2, 0, UnlockCurrency::XP);
    let true_evol = EvolutionInfo::new(EVOL_ID, EvolutionType::Other);
    let costs = [
        8_000, 10_000, 16_000, 24_000, 34_000, 46_000, 60_000, 76_000, 94_000, 114_000,
    ];
    let misc = Misc {
        rarity: Rarity::Special,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: 1,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        bahamut,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: SPECIAL_MAX,
            misc
        }
    );
}

#[test]
fn cancan() {
    const EVOL_ID: u32 = 15033;
    const SELL_XP: u32 = 478_000;
    const SELL_NP: u8 = 15;

    let version = TEST_CONFIG.version.current_version();

    let cancan = get_unitbuy(32, version);
    let guide_cmp_const = cancan.misc.guide_order.clone();

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
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        cancan,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: SR_MAX,
            misc
        }
    );
}

#[test]
fn dio() {
    const TRUE_ID: u32 = 15178;
    const ULTRA_ID: u32 = 25178;
    const SELL_XP: u32 = 1_000_000;
    const SELL_NP: u8 = 50;

    let version = TEST_CONFIG.version.current_version();

    let dio = get_unitbuy(177, version);
    let guide_cmp_const = dio.misc.guide_order.clone();

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

    let costs = UR_COSTS;
    let misc = Misc {
        rarity: Rarity::UberRare,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        dio,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: Some(ultra_evol),
            upgrade_costs: UpgradeCost { costs },
            max_levels: UR_MAX,
            misc
        }
    );
}

#[test]
fn metal() {
    const SELL_XP: u32 = 777_777;
    const SELL_NP: u8 = 15;

    let version = TEST_CONFIG.version.current_version();

    let metal = get_unitbuy(200, version);
    let guide_cmp_const = metal.misc.guide_order.clone();

    let unlock = CatUnlock::default();

    let costs = UR_COSTS;
    let misc = Misc {
        rarity: Rarity::SuperRare,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: 0,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        metal,
        UnitBuy {
            unlock,
            true_evol: None,
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: MaxLevels {
                ch1: 10,
                ch2: 20,
                initial_plus: 0,
                max_nat: 20,
                max_plus: 0
            },
            misc
        }
    );
}

#[test]
fn dasli() {
    const SELL_XP: u32 = 1_000_000;
    const SELL_NP: u8 = 50;
    const UPDATE: i64 = 90500;

    let version = TEST_CONFIG.version.current_version();

    let dasli = get_unitbuy(543, version);
    let guide_cmp_const = dasli.misc.guide_order.clone();

    let unlock = CatUnlock::default();

    let costs = UR_COSTS;
    let misc = Misc {
        rarity: Rarity::UberRare,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: UPDATE,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        dasli,
        UnitBuy {
            unlock,
            true_evol: None,
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: UR_MAX,
            misc
        }
    );
}

#[test]
fn modoki() {
    const SELL_XP: u32 = 10000;
    const SELL_NP: u8 = 1;

    let version = TEST_CONFIG.version.current_version();

    let modoki = get_unitbuy(626, version);
    let guide_cmp_const = modoki.misc.guide_order.clone();
    let update_cmp_const = modoki.misc.update_released;
    // -1 in en but actual version in jp

    let unlock = CatUnlock::default();

    let costs = [
        3_500, 5_600, 8_540, 12_460, 17_360, 23_240, 30_100, 37_940, 46_760, 56_560,
    ];
    let misc = Misc {
        rarity: Rarity::Special,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: update_cmp_const,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        modoki,
        UnitBuy {
            unlock,
            true_evol: None,
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: MaxLevels {
                ch1: 1,
                ch2: 1,
                initial_plus: 30,
                max_nat: 1,
                max_plus: 30
            },
            misc
        }
    );
}

#[test]
fn superfeline() {
    const SELL_XP: u32 = 30000;
    const SELL_NP: u8 = 1;
    const UPDATE: i64 = 110_100;

    let version = TEST_CONFIG.version.current_version();

    let sfeline = get_unitbuy(643, version);
    let guide_cmp_const = sfeline.misc.guide_order.clone();

    let unlock = CatUnlock::default();
    let true_evol = EvolutionInfo::new(19010, EvolutionType::Levels { level: 100 });
    let costs = [
        999_999, 999_999, 999_999, 999_999, 999_999, 999_999, 999_999, 999_999, 999_999, 999_999,
    ];
    let misc = Misc {
        rarity: Rarity::Normal,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: UPDATE,
        egg_info: AncientEggInfo::None,
    };

    assert_eq!(
        sfeline,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: MaxLevels {
                initial_plus: 9,
                ..NORMAL_MAX
            },
            misc
        }
    );
}

#[test]
fn courier() {
    const EVOL_ID: u32 = 15659;
    const SELL_XP: u32 = 444;
    const SELL_NP: u8 = 5;
    const UPDATE: i64 = 110_500;

    let version = TEST_CONFIG.version.current_version();

    let courier = get_unitbuy(658, version);
    let guide_cmp_const = courier.misc.guide_order.clone();

    let unlock = CatUnlock::new(0, 0, 0, UnlockCurrency::XP);

    let tf_costs = [
        evol_item(I::PurpleStone, 5),
        evol_item(I::RedStone, 5),
        evol_item(I::Nothing, 0),
        evol_item(I::Nothing, 0),
        evol_item(I::Nothing, 0),
    ];
    let true_evol = EvolutionInfo::new(
        EVOL_ID,
        EvolutionType::Catfruit(CatfruitEvolution {
            item_cost: tf_costs,
            xp_cost: 200_000,
            level_required: 30,
        }),
    );

    let costs = [
        2_000, 3_500, 6_200, 9_800, 14_300, 19_700, 26_000, 33_200, 41_300, 50_300,
    ];
    let misc = Misc {
        rarity: Rarity::Rare,
        guide_order: guide_cmp_const,
        sell_xp: SELL_XP,
        sell_np: SELL_NP,
        update_released: UPDATE,
        egg_info: AncientEggInfo::Egg {
            normal: 0,
            evolved: 2,
        },
    };

    assert_eq!(
        courier,
        UnitBuy {
            unlock,
            true_evol: Some(true_evol),
            ultra_evol: None,
            upgrade_costs: UpgradeCost { costs },
            max_levels: SPECIAL_MAX,
            misc
        }
    );
}

#[test]
fn make_sure_category_works() {
    for rarity in Rarity::iter() {
        assert_eq!(
            rarity.category(),
            &format!("[[:Category:{r} Cats|{r} Cat]]", r = rarity.as_str())
        )
    }
}
