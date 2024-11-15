//! Deals with enemy encounters.

use crate::{
    config::Config,
    data::stage::{
        parsed::stage_enemy::Magnification,
        raw::{
            stage_data::{csv_types::StageEnemyCSV, StageData},
            stage_metadata::StageMeta,
        },
    },
};
use regex::Regex;

fn filter_map_stage(
    abs_enemy_id: u32,
    file_name: String,
    config: &Config,
) -> Option<StageData<'_>> {
    let stage = StageData::new(&file_name, &config.current_version).unwrap();

    if !stage
        .stage_csv_data
        .enemies
        .iter()
        .any(|e| e.num == abs_enemy_id)
    {
        return None;
    }

    Some(stage)
}

fn get_encounters(wiki_enemy_id: u32, config: &Config) -> Vec<StageData<'_>> {
    let stage_file_re = Regex::new(r"^stage.*?\d{2}\.csv$").unwrap();
    let dir = &config.current_version.get_file_path("DataLocal");
    let abs_enemy_id = wiki_enemy_id + 2;

    let files = std::fs::read_dir(dir).unwrap();
    let encounters = files.filter_map(|f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();
        if !stage_file_re.is_match(&file_name) {
            return None;
        };

        filter_map_stage(abs_enemy_id, file_name, &config)
    });

    encounters.collect()
}

/// Do thing (temp)
pub fn do_thing(config: Config) {
    let wiki_enemy_id = 703;

    println!("{:?}", get_encounters(wiki_enemy_id, &config));
    todo!()
}

/*
Due to how the encounters section is constantly evolving, `get_encounters`
cannot be tested any other way than empirically.
*/
