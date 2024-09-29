//! Deals with the stage's rewards.

use crate::{
    data::{
        map::map_data::csv_types::{TreasureCSV, TreasureType as T},
        stage::parsed::stage::{Stage, StageRewards},
    },
    wikitext::{data_files::rewards::Treasure_data, template_parameter::TemplateParameter},
};
use num_format::{Locale, WriteFormatted};
use std::io::Write;

#[inline]
fn write_standard_chance(buf: &mut Vec<u8>, data: &TreasureCSV) {
    if data.item_id == 6 {
        // XP is a special case from the rest
        buf.write_formatted(&data.item_amt, &Locale::en);
        write!(buf, " {}", Treasure_data.get_treasure_name(data.item_id)).unwrap();
        return;
    }
    write!(buf, "{} +", Treasure_data.get_treasure_name(data.item_id)).unwrap();
    buf.write_formatted(&data.item_amt, &Locale::en);
}

fn OnceThenUnlimited(rewards: &StageRewards) -> Vec<u8> {
    let mut buf = vec![];
    let t = &rewards.treasure_drop;

    buf.write(b"- ").unwrap();
    write_standard_chance(&mut buf, &t[0]);
    write!(buf, " ({}%, 1 time)", t[0].item_chance).unwrap();

    for item in &t[1..] {
        if item.item_chance == 0 {
            continue;
        }
        todo!()
    }
    buf
}

pub fn treasure(stage: &Stage) -> Option<TemplateParameter> {
    let rewards = stage.rewards.as_ref()?;

    let treasure_text = match rewards.treasure_type {
        T::OnceThenUnlimited => OnceThenUnlimited(rewards),
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
