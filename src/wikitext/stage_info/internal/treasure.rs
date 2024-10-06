//! Deals with the stage's rewards.

use crate::{
    data::{
        map::map_data::csv_types::TreasureType as T,
        stage::parsed::stage::{Stage, StageRewards},
    },
    wikitext::{data_files::rewards::TREASURE_DATA, template_parameter::TemplateParameter},
};
use num_format::{Locale, WriteFormatted};
use std::fmt::Write;

#[inline]
fn is_unit_drop(id: u32) -> bool {
    1_000 <= id && id < 30_000
}

/// Write item name and amount e.g. `50,000 XP` or `Treasure Radar +1`.
fn write_name_and_amount(buf: &mut String, id: u32, amt: u32) {
    if id == 6 {
        // XP is a special case from the rest
        buf.write_formatted(&amt, &Locale::en).unwrap();
        write!(buf, " {}", TREASURE_DATA.get_treasure_name(id)).unwrap();
        return;
    }

    if is_unit_drop(id) {
        *buf += TREASURE_DATA.get_treasure_name(id);
        return;
    }

    write!(buf, "{} +", TREASURE_DATA.get_treasure_name(id)).unwrap();
    buf.write_formatted(&amt, &Locale::en).unwrap();
}

/// When treasure type is first item drops once then rest are all unlimited.
fn once_then_unlimited(rewards: &StageRewards) -> String {
    let mut buf = "".to_string();
    let t = &rewards.treasure_drop;

    buf.write_str("- ").unwrap();
    write_name_and_amount(&mut buf, t[0].item_id, t[0].item_amt);
    write!(buf, " ({}%, 1 time)", t[0].item_chance).unwrap();

    let mut total_allowed: f64 = 100.0;
    for item in &t[1..] {
        if item.item_chance == 0 {
            continue;
        }
        buf.write_str("<br>\n- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);

        let chance = total_allowed * f64::from(item.item_chance) / 100.0;
        total_allowed -= chance;
        let precision = if chance % 1.0 == 0.0 { 0 } else { 1 };
        let limit = if is_unit_drop(item.item_id) {
            "1 time"
        } else {
            "unlimited"
        };
        write!(buf, " ({:.1$}%, {limit})", chance, precision).unwrap();
    }
    buf
}

/// When treasure type is that all items have unlimited drop potential.
fn all_unlimited(rewards: &StageRewards) -> String {
    let mut buf = "".to_string();
    let t = &rewards.treasure_drop;

    let mut total_allowed: f64 = 100.0;
    for item in t {
        if item.item_chance == 0 {
            continue;
        }
        buf.write_str("- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);

        let chance = total_allowed * f64::from(item.item_chance) / 100.0;
        total_allowed -= chance;
        let precision = if chance % 1.0 == 0.0 { 0 } else { 1 };
        let limit = if is_unit_drop(item.item_id) {
            "1 time"
        } else {
            "unlimited"
        };
        write!(buf, " ({:.1$}%, {limit})", chance, precision).unwrap();
        buf.write_str("<br>\n").unwrap();
    }

    if buf.is_empty() {
        "".to_string()
    } else {
        buf.truncate(buf.len() - "<br>\n".len());
        buf
    }
}

fn single_raw(rewards: &StageRewards) -> String {
    let t = &rewards.treasure_drop;
    assert_eq!(t.len(), 1);
    if t[0].item_chance == 0 {
        return "".to_string();
    }

    todo!();
    // let mut buf = "".to_string();
    // buf.write_str("- ").unwrap();
    // write_name_and_amount(&mut buf, t[0].item_id, t[0].item_amt);
    // write!(buf, " ({}%, 1 time)", t[0].item_chance).unwrap();

    // buf
}

/// When treasure type is that a treasure is guaranteed but can only be received
/// once.
fn guaranteed_once(rewards: &StageRewards) -> String {
    let mut buf = "".to_string();
    let t = &rewards.treasure_drop;
    if t.len() == 1 {
        buf.write_str("- ").unwrap();
        write_name_and_amount(&mut buf, t[0].item_id, t[0].item_amt);
        buf.write_str(" (100%, 1 time)").unwrap();
        return buf;
    };

    buf.write_str("One of the following (1 time):").unwrap();
    for item in t {
        buf.write_str("<br>\n- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);
    }

    buf
}

/// When treasure type is that a treasure is guaranteed and the stage has
/// unlimited drops.
fn guaranteed_unlimited(rewards: &StageRewards) -> String {
    let mut buf = "".to_string();
    let t = &rewards.treasure_drop;
    if t.len() == 1 {
        todo!()
        // buf.write_str("- ").unwrap();
        // write_name_and_amount(&mut buf, &t[0].item_id, &t[0].item_amt);
        // buf.write_str(" (100%, 1 time)").unwrap();
        // return buf;
    };

    buf.write_str("One of the following (unlimited):").unwrap();
    for item in t {
        buf.write_str("<br>\n- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);
    }

    buf
}

/// Get the `treasure` section of Stage Info.
pub fn treasure(stage: &Stage) -> Option<TemplateParameter> {
    let rewards = stage.rewards.as_ref()?;

    let treasure_text = match rewards.treasure_type {
        T::OnceThenUnlimited => once_then_unlimited(rewards),
        T::AllUnlimited => all_unlimited(rewards),
        T::UnclearMaybeRaw => single_raw(rewards),
        T::GuaranteedOnce => guaranteed_once(rewards),
        T::GuaranteedUnlimited => guaranteed_unlimited(rewards),
    };

    if treasure_text.is_empty() {
        None
    } else {
        Some(TemplateParameter::new("treasure", treasure_text))
    }
}

/// Get the `score reward` section of Stage Info.
pub fn score_rewards(stage: &Stage) -> Option<TemplateParameter> {
    let rewards = &stage.rewards.as_ref()?.score_rewards;
    if rewards.is_empty() {
        return None;
    }

    let scores = rewards
        .iter()
        .map(|r| {
            let mut buf = "".to_string();
            buf.write_str("'''").unwrap();
            buf.write_formatted(&r.score, &Locale::en).unwrap();
            buf.write_str("''': ").unwrap();
            write_name_and_amount(&mut buf, r.item_id, r.item_amt);
            buf
        })
        .collect::<Vec<String>>()
        .join("<br>\n");

    Some(TemplateParameter::new("score reward", scores))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::map::map_data::csv_types::{TreasureCSV, TreasureType};

    #[test]
    fn write_name_and_amount_normal() {
        const CAT_FOOD: u32 = 22;
        let mut buf = "".to_string();
        write_name_and_amount(&mut buf, CAT_FOOD, 22_222);
        assert_eq!(buf, "[[Cat Food]] +22,222");
    }

    #[test]
    fn write_name_and_amount_xp() {
        const XP: u32 = 6;
        let mut buf = "".to_string();
        write_name_and_amount(&mut buf, XP, 40_000);
        assert_eq!(buf, "40,000 XP");
    }

    #[test]
    fn write_name_and_amount_unit() {
        const CRAZED_CAT: u32 = 1_103;
        let mut buf = "".to_string();
        write_name_and_amount(&mut buf, CRAZED_CAT, 40_000);
        assert_eq!(buf, "[[Crazed Cat (Super Rare Cat)|Crazed Cat]]");
    }

    #[test]
    fn write_name_and_amount_tf() {
        const MANIC_MOHAWK: u32 = 10_092;
        let mut buf = "".to_string();
        write_name_and_amount(&mut buf, MANIC_MOHAWK, 40_000);
        assert_eq!(
            buf,
            "[[Crazed Cat (Super Rare Cat)|Crazed Cat]]'s [[True Form]]"
        );
    }

    #[test]
    fn write_name_and_amount_orb() {
        const RED_ATTACK_ORB: u32 = 30_000;
        let mut buf = "".to_string();
        write_name_and_amount(&mut buf, RED_ATTACK_ORB, 1);
        assert_eq!(buf, "Attack Up D [[Talent Orbs|Orb]]: Red +1");
    }

    #[test]
    fn once_then_nothing() {
        let ht30 = Stage::new("v 0 29").unwrap();
        assert_eq!(
            treasure(&ht30),
            Some(TemplateParameter::new(
                "treasure",
                "- [[Cat Capsule#Rare Cat Capsule|Rare Ticket]] +3 (100%, 1 time)".to_string()
            ))
        );
        assert_eq!(score_rewards(&ht30), None);
    }

    #[test]
    fn unit_reward() {
        let dark_souls = Stage::new("s 17 0").unwrap();
        assert_eq!(
            treasure(&dark_souls),
            Some(TemplateParameter::new(
                "treasure",
                "- [[Crazed Cat (Super Rare Cat)|Crazed Cat]] (100%, 1 time)".to_string()
            ))
        );
        assert_eq!(score_rewards(&dark_souls), None);
    }

    #[test]
    fn once_then_many() {
        let merciless_xp = Stage::new("s 155 0").unwrap();
        assert_eq!(
            treasure(&merciless_xp),
            Some(TemplateParameter::new(
                "treasure",
                "- 2,030,000 XP (10%, 1 time)<br>\n\
                - 1,020,000 XP (30%, unlimited)<br>\n\
                - 510,000 XP (70%, unlimited)"
                    .to_string()
            ))
        );
        assert_eq!(score_rewards(&merciless_xp), None);
    }

    #[test]
    fn many_unlimited() {
        let jubilee_night = Stage::new("ex 1 0").unwrap();
        assert_eq!(
            treasure(&jubilee_night),
            Some(TemplateParameter::new(
                "treasure",
                "- [[Catfruit|Epic Catfruit]] +1 (70%, unlimited)<br>\n\
                - [[Catfruit|Purple Catfruit]] +1 (25.5%, unlimited)<br>\n\
                - [[Catfruit|Purple Catfruit Seed]] +1 (4.5%, unlimited)"
                    .to_string()
            ))
        );
        assert_eq!(score_rewards(&jubilee_night), None);
    }

    #[test]
    fn treasure_radar() {
        let round_4_trust_fund = Stage::new("sr 0 3").unwrap();
        assert_eq!(
            round_4_trust_fund.rewards,
            Some(StageRewards {
                treasure_type: T::UnclearMaybeRaw,
                treasure_drop: vec![TreasureCSV {
                    item_chance: 0,
                    item_id: 1,
                    item_amt: 1 // treasure radar with 0 chance
                }],
                score_rewards: vec![]
            })
        );
        assert_eq!(treasure(&round_4_trust_fund), None);
    }

    #[test]
    fn guaranteed_once_single() {
        let it30 = Stage::new("v 6 29").unwrap();
        assert_eq!(
            treasure(&it30),
            Some(TemplateParameter::new(
                "treasure",
                "- [[Catfruit|Gold Catfruit Seed]] +1 (100%, 1 time)".to_string()
            ))
        );
        assert_eq!(score_rewards(&it30), None);
    }

    #[test]
    fn guaranteed_once_many() {
        let it29 = Stage::new("v 6 28").unwrap();
        assert_eq!(
            treasure(&it29),
            Some(TemplateParameter::new(
                "treasure",
                "One of the following (1 time):<br>\n\
                - Bricks +5<br>\n\
                - Feathers +5<br>\n\
                - Coal +5<br>\n\
                - Sprockets +5<br>\n\
                - Gold +5<br>\n\
                - Meteorite +5<br>\n\
                - Beast Bones +5<br>\n\
                - Ammonite +5"
                    .to_string()
            ))
        );
        assert_eq!(score_rewards(&it29), None);
    }
    // other it one with 33, 34, 33 (it20?)

    #[test]
    fn guaranteed_unlimited_many() {
        let sakura_dance = Stage::new("c 128 2").unwrap();
        assert_eq!(
            treasure(&sakura_dance),
            Some(TemplateParameter::new(
                "treasure",
                "One of the following (unlimited):<br>\n\
                - [[Battle Items#Speed Up|Speed Up]] +4<br>\n\
                - [[Battle Items#Treasure Radar|Treasure Radar]] +2<br>\n\
                - [[Battle Items#Rich Cat|Rich Cat]] +2<br>\n\
                - [[Battle Items#Cat CPU|Cat CPU]] +4<br>\n\
                - [[Battle Items#Cat Jobs|Cat Jobs]] +2<br>\n\
                - [[Battle Items#Sniper the Cat|Sniper the Cat]] +2"
                    .to_string()
            ))
        );
        assert_eq!(score_rewards(&sakura_dance), None);
    }

    #[test]
    fn labyrinth() {
        let labyrinth_67 = Stage::new("l 0 66").unwrap();
        assert_eq!(labyrinth_67.rewards, None);
        assert_eq!(treasure(&labyrinth_67), None);
        assert_eq!(score_rewards(&labyrinth_67), None);
    }

    #[test]
    fn score_basic() {
        let korea = Stage::new("itf 1 1").unwrap();
        assert_eq!(
            score_rewards(&korea),
            Some(TemplateParameter::new(
                "score reward",
                "'''8,500''': [[Cat Food]] +10<br>\n\
                '''5,000''': 25,000 XP"
                    .to_string()
            ))
        );
    }

    #[test]
    fn radar_impossible() {
        let explosion_in_sky = Stage::new("s 112 0").unwrap();
        assert_eq!(
            explosion_in_sky.rewards,
            Some(StageRewards {
                treasure_type: TreasureType::AllUnlimited,
                treasure_drop: vec![TreasureCSV {
                    item_chance: 0,
                    item_id: 1,
                    item_amt: 1
                }],
                score_rewards: vec![]
            })
        );
        assert_eq!(treasure(&explosion_in_sky), None)
    }
}
