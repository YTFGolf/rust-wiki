//! Deals with the stage's rewards.

use crate::{
    data::{
        map::map_data::csv_types::{TreasureCSV, TreasureType as T},
        stage::parsed::stage::{Stage, StageRewards},
    },
    wikitext::{data_files::rewards::TREASURE_DATA, template_parameter::TemplateParameter},
};
use num_format::{Locale, WriteFormatted};
use std::io::Write;

fn write_name_and_amount(buf: &mut Vec<u8>, data: &TreasureCSV) {
    if data.item_id == 6 {
        // XP is a special case from the rest
        buf.write_formatted(&data.item_amt, &Locale::en).unwrap();
        write!(buf, " {}", TREASURE_DATA.get_treasure_name(data.item_id)).unwrap();
        return;
    }

    write!(buf, "{} +", TREASURE_DATA.get_treasure_name(data.item_id)).unwrap();
    buf.write_formatted(&data.item_amt, &Locale::en).unwrap();
}

fn once_then_unlimited(rewards: &StageRewards) -> Vec<u8> {
    let mut buf = vec![];
    let t = &rewards.treasure_drop;

    buf.write(b"- ").unwrap();
    write_name_and_amount(&mut buf, &t[0]);
    let mut total_allowed: f64 = 100.0;
    write!(buf, " ({}%, 1 time)", t[0].item_chance).unwrap();

    for item in &t[1..] {
        if item.item_chance == 0 {
            continue;
        }
        buf.write(b"<br>\n- ").unwrap();
        write_name_and_amount(&mut buf, &item);

        let chance = total_allowed * f64::from(item.item_chance) / 100.0;
        total_allowed -= chance;
        let precision = if chance % 1.0 == 0.0 { 0 } else { 1 };
        write!(buf, " ({:.1$}%, unlimited)", chance, precision).unwrap();
    }
    buf
}

pub fn treasure(stage: &Stage) -> Option<TemplateParameter> {
    let rewards = stage.rewards.as_ref()?;

    let treasure_text = match rewards.treasure_type {
        T::OnceThenUnlimited => once_then_unlimited(rewards),
        _ => todo!(),
    };

    Some(TemplateParameter::new(b"treasure", treasure_text))
}

// TODO move to different file
pub fn score_rewards(stage: &Stage) -> Option<TemplateParameter> {
    let rewards = match &stage.rewards {
        None => return None,
        Some(t) => t,
    };
    println!("{:?}", rewards);
    None

    // todo!()
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
        assert_eq!(
            treasure(&it29),
            Some(TemplateParameter::new(
                b"treasure",
                b"One of the following (1 time):\n\
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
                b"One of the following (unlimited):\n\
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
        assert_eq!(treasure(&labyrinth_67), None);
    }
}
