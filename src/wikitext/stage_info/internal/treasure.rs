//! Deals with the stage's rewards.

use crate::{
    data::{
        map::map_data::csv_types::TreasureType as T,
        stage::parsed::stage::{Stage, StageRewards},
    },
    wikitext::{data_files::rewards::TREASURE_DATA, template_parameter::TemplateParameter},
};
use num_format::{Locale, WriteFormatted};
use std::io::Write;

/// Write item name and amount e.g. `50,000 XP` or `Treasure Radar +1`.
fn write_name_and_amount(buf: &mut Vec<u8>, id: u32, amt: u32) {
    if id == 6 {
        // XP is a special case from the rest
        buf.write_formatted(&amt, &Locale::en).unwrap();
        write!(buf, " {}", TREASURE_DATA.get_treasure_name(id)).unwrap();
        return;
    }

    write!(buf, "{} +", TREASURE_DATA.get_treasure_name(id)).unwrap();
    buf.write_formatted(&amt, &Locale::en).unwrap();
}

/// When treasure type is first item drops once then rest are all unlimited.
fn once_then_unlimited(rewards: &StageRewards) -> Vec<u8> {
    let mut buf = vec![];
    let t = &rewards.treasure_drop;

    buf.write(b"- ").unwrap();
    write_name_and_amount(&mut buf, t[0].item_id, t[0].item_amt);
    write!(buf, " ({}%, 1 time)", t[0].item_chance).unwrap();

    let mut total_allowed: f64 = 100.0;
    for item in &t[1..] {
        if item.item_chance == 0 {
            continue;
        }
        buf.write(b"<br>\n- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);

        let chance = total_allowed * f64::from(item.item_chance) / 100.0;
        total_allowed -= chance;
        let precision = if chance % 1.0 == 0.0 { 0 } else { 1 };
        write!(buf, " ({:.1$}%, unlimited)", chance, precision).unwrap();
    }
    buf
}

/// When treasure type is that all items have unlimited drop potential.
fn all_unlimited(rewards: &StageRewards) -> Vec<u8> {
    let mut buf = vec![];
    let t = &rewards.treasure_drop;

    let mut total_allowed: f64 = 100.0;
    for item in t {
        if item.item_chance == 0 {
            continue;
        }
        buf.write(b"- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);

        let chance = total_allowed * f64::from(item.item_chance) / 100.0;
        total_allowed -= chance;
        let precision = if chance % 1.0 == 0.0 { 0 } else { 1 };
        write!(buf, " ({:.1$}%, unlimited)", chance, precision).unwrap();
        buf.write(b"<br>\n").unwrap();
    }

    buf.truncate(buf.len() - "<br>\n".len());
    buf
}

/// When treasure type is that a treasure is guaranteed but can only be received
/// once.
fn guaranteed_once(rewards: &StageRewards) -> Vec<u8> {
    let mut buf = vec![];
    let t = &rewards.treasure_drop;
    if t.len() == 1 {
        buf.write(b"- ").unwrap();
        write_name_and_amount(&mut buf, t[0].item_id, t[0].item_amt);
        buf.write(b" (100%, 1 time)").unwrap();
        return buf;
    };

    buf.write(b"One of the following (1 time):").unwrap();
    for item in t {
        buf.write(b"<br>\n- ").unwrap();
        write_name_and_amount(&mut buf, item.item_id, item.item_amt);
    }

    buf
}

/// When treasure type is that a treasure is guaranteed and the stage has
/// unlimited drops.
fn guaranteed_unlimited(rewards: &StageRewards) -> Vec<u8> {
    let mut buf = vec![];
    let t = &rewards.treasure_drop;
    if t.len() == 1 {
        todo!()
        // buf.write(b"- ").unwrap();
        // write_name_and_amount(&mut buf, &t[0].item_id, &t[0].item_amt);
        // buf.write(b" (100%, 1 time)").unwrap();
        // return buf;
    };

    buf.write(b"One of the following (unlimited):").unwrap();
    for item in t {
        buf.write(b"<br>\n- ").unwrap();
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
        // -1 => TreasureType::UnclearMaybeRaw,
        T::GuaranteedOnce => guaranteed_once(rewards),
        T::GuaranteedUnlimited => guaranteed_unlimited(rewards),
        _ => todo!(),
    };

    Some(TemplateParameter::new(b"treasure", treasure_text))
}

/// Get the `score reward` section of Stage Info.
pub fn score_rewards(stage: &Stage) -> Option<TemplateParameter> {
    let rewards = &stage.rewards.as_ref()?.score_rewards;

    let scores = rewards
        .iter()
        .map(|r| {
            let mut buf = vec![];
            buf.write(b"'''").unwrap();
            buf.write_formatted(&r.score, &Locale::en).unwrap();
            buf.write(b"''': ").unwrap();
            write_name_and_amount(&mut buf, r.item_id, r.item_amt);
            buf
        })
        .collect::<Vec<Vec<u8>>>()
        .join(b"<br>\n".as_slice());

    Some(TemplateParameter::new(b"score reward", scores))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn once_then_nothing() {
        let ht30 = Stage::new("v 0 29").unwrap();
        assert_eq!(
            treasure(&ht30),
            Some(TemplateParameter::new(
                b"treasure",
                b"- [[Cat Capsule#Rare Cat Capsule|Rare Ticket]] +3 (100%, 1 time)".to_vec()
            ))
        );
    }

    #[test]
    fn once_then_many() {
        let merciless_xp = Stage::new("s 155 0").unwrap();
        println!("{:?}", String::from(treasure(&merciless_xp).unwrap()));
        assert_eq!(
            treasure(&merciless_xp),
            Some(TemplateParameter::new(
                b"treasure",
                b"- 2,030,000 XP (10%, 1 time)<br>\n\
                - 1,020,000 XP (30%, unlimited)<br>\n\
                - 510,000 XP (70%, unlimited)"
                    .to_vec()
            ))
        );
    }

    #[test]
    fn many_unlimited() {
        let jubilee_night = Stage::new("ex 1 0").unwrap();
        println!("{:?}", String::from(treasure(&jubilee_night).unwrap()));
        assert_eq!(
            treasure(&jubilee_night),
            Some(TemplateParameter::new(
                b"treasure",
                b"- [[Catfruit|Epic Catfruit]] +1 (70%, unlimited)<br>\n\
                - [[Catfruit|Purple Catfruit]] +1 (25.5%, unlimited)<br>\n\
                - [[Catfruit|Purple Catfruit Seed]] +1 (4.5%, unlimited)"
                    .to_vec()
            ))
        );
    }

    #[test]
    fn guaranteed_once_single() {
        let it30 = Stage::new("v 6 29").unwrap();
        assert_eq!(
            treasure(&it30),
            Some(TemplateParameter::new(
                b"treasure",
                b"- [[Catfruit|Gold Catfruit Seed]] +1 (100%, 1 time)".to_vec()
            ))
        )
    }

    #[test]
    fn guaranteed_once_many() {
        let it29 = Stage::new("v 6 28").unwrap();
        println!("{:?}", String::from(treasure(&it29).unwrap()));
        assert_eq!(
            treasure(&it29),
            Some(TemplateParameter::new(
                b"treasure",
                b"One of the following (1 time):<br>\n\
                - Bricks +5<br>\n\
                - Feathers +5<br>\n\
                - Coal +5<br>\n\
                - Sprockets +5<br>\n\
                - Gold +5<br>\n\
                - Meteorite +5<br>\n\
                - Beast Bones +5<br>\n\
                - Ammonite +5"
                    .to_vec()
            ))
        );
    }
    // other it one with 33, 34, 33 (it20?)

    #[test]
    fn guaranteed_unlimited_many() {
        let sakura_dance = Stage::new("c 128 2").unwrap();
        assert_eq!(
            treasure(&sakura_dance),
            Some(TemplateParameter::new(
                b"treasure",
                b"One of the following (unlimited):<br>\n\
                - [[Battle Items#Speed Up|Speed Up]] +4<br>\n\
                - [[Battle Items#Treasure Radar|Treasure Radar]] +2<br>\n\
                - [[Battle Items#Rich Cat|Rich Cat]] +2<br>\n\
                - [[Battle Items#Cat CPU|Cat CPU]] +4<br>\n\
                - [[Battle Items#Cat Jobs|Cat Jobs]] +2<br>\n\
                - [[Battle Items#Sniper the Cat|Sniper the Cat]] +2"
                    .to_vec()
            ))
        );
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
                b"score reward",
                b"'''8,500''': [[Cat Food]] +10<br>\n\
                '''5,000''': 25,000 XP"
                    .to_vec()
            ))
        );
    }
}
