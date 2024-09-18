//! Module that deals with getting information about stages.
use super::stage_option::{StageOptionCSV, STAGE_OPTION};
use crate::{
    file_handler::get_decommented_file_reader,
    map::{
        map_data::{csv_types::StageDataCSV, GameMap},
        map_option::{MapOptionCSV, MAP_OPTION},
    },
    stage::stage_metadata::StageMeta,
};
use csv_types::*;
use std::path::PathBuf;

/// Types to deserialise csv files.
pub mod csv_types {
    #[derive(Debug, serde::Deserialize)]
    /// Data stored in the header of the csv file (minus most Main Chapters).
    pub struct HeaderCSV {
        /// ID of base used.
        pub base_id: i32,
        /// Is no continues? Boolean value.
        pub no_cont: u32,
        /// % chance of continuation.
        pub cont_chance: u32,
        /// `map_num` of continuation stages.
        pub contmap_id: u32,
        /// Minimum `stage_num` of any continuation stage.
        pub cont_stage_idmin: u32,
        /// Maximum `stage_num` of any continuation stage.
        pub cont_stage_idmax: u32,
    }

    #[derive(Debug, serde::Deserialize)]
    /// Data stored in line 2 of the csv file (line 1 for most Main Chapter
    /// stages).
    pub struct Line2CSV {
        /// Stage width.
        pub width: u32,
        /// Base HP (ignore this if `animbase_id` is not 0).
        pub base_hp: u32,
        _unknown_1: u32,
        _unknown_2: u32,
        /// ID of stage background.
        pub background_id: u32,
        /// Max enemies in stage.
        pub max_enemies: u32,
        /// ID of animated base (if 0 then no base).
        pub animbase_id: u32,
        /// Time limit (is this only used in Dojo stages?).
        pub time_limit: u32,
        /// Do you have the green barrier thing (boolean value).
        pub indestructible: u32,
        _unknown_3: Option<u32>,
    }

    #[derive(Debug, serde::Deserialize)]
    /// CSV data for enemies. See [Stage Structure
    /// Page/Battlegrounds](https://battle-cats.fandom.com/wiki/Battle_Cats_Wiki:Stage_Structure_Page/Battlegrounds)
    /// for more complete documentation.
    pub struct StageEnemyCSV {
        /// battle-cats db id (i.e. Doge is 2).
        pub num: u32,
        /// Amount of enemy that spawns (0 = infinite).
        pub amt: u32,
        /// Start frame of enemies / 2. Ignored (unless `is_spawn_delay` is
        /// true) for enemies that spawn after base hit.
        pub start_frame: u32,
        /// Min respawn frame of enemies / 2.
        pub respawn_frame_min: u32,
        /// Max respawn frame of enemies / 2.
        pub respawn_frame_max: u32,
        /// At what percentage does the enemy spawn (absolute value for Dojo).
        pub base_hp: u32,
        /// Minimum layer.
        pub layer_min: u32,
        /// Maximum layer.
        pub layer_max: u32,
        /// 0 = none, 1 = boss, 2 = with screen shake.
        pub boss_type: u32,
        /// Enemy magnification.
        pub magnification: Option<u32>,

        #[serde(default)]
        _unknown_1: Option<u32>,
        #[serde(default)]
        /// If not 0 then enemy has different hp and ap mags. `magnification`
        /// should be taken as hp mag and this is ap mag.
        pub attack_magnification: Option<u32>,
        #[serde(default)]
        /// If base hp is <100 (unsure what the effect is in Dojos) and this is
        /// 1 then `start_frame` is not ignored (boolean value).
        pub is_spawn_delay: Option<u32>,
        #[serde(default)]
        /// How many cats need to die before enemy spawns.
        pub kill_count: Option<u32>,
    }

    /// Raw data from the stage csv file.
    #[derive(Debug)]
    pub struct RawCSVData {
        /// Header row.
        pub header: HeaderCSV,
        /// Line 2.
        pub line2: Line2CSV,
        /// Enemies.
        pub enemies: Vec<StageEnemyCSV>,
    }
}

/// Stores information about a stage.
pub struct StageData {
    /// Stage's metadata.
    pub meta: StageMeta,
    /// Data stored in the stage's CSV file.
    pub stage_csv_data: RawCSVData,
}

impl StageData {
    pub fn new(selector: &str) -> Option<Self> {
        let meta = StageMeta::new(selector).unwrap();

        let stage_file = PathBuf::from("DataLocal").join(&meta.stage_file_name);
        let stage_file_reader = get_decommented_file_reader(stage_file).unwrap();
        let stage_csv_data = Self::read_stage_csv(stage_file_reader);

        Some(StageData {
            meta,
            stage_csv_data,
        })
    }

    /// Read a stage's csv file and obtain the data from it.
    pub fn read_stage_csv<R: std::io::Read>(reader: R) -> RawCSVData {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            // .from_reader(File::open(gd.join("DataLocal/stage00.csv")).unwrap())
            // .from_reader(File::open(gd.join("DataLocal/stage.csv")).unwrap())
            .from_reader(reader);

        let mut records = rdr.byte_records();

        let mut head = records.next().unwrap().unwrap();
        let csv_head: HeaderCSV = if head.len() <= 7 || head[6].is_empty() {
            let tmp = head;
            head = records.next().unwrap().unwrap();
            tmp.deserialize(None).unwrap()
            // if (cas == -1)
            //     cas = CH_CASTLES[id.id];
        } else {
            // In EoC
            HeaderCSV {
                base_id: 0,
                no_cont: 0,
                cont_chance: 0,
                contmap_id: 0,
                cont_stage_idmin: 0,
                cont_stage_idmax: 0,
            }
            // castle = Identifier.parseInt(sm.cast * 1000 + CH_CASTLES[id.id], CastleImg.class);
        };
        let line_2 = head;
        let csv_line_2: Line2CSV = line_2.deserialize(None).unwrap();

        let mut enemies = vec![];
        for result in rdr.byte_records() {
            let record: StageEnemyCSV = match result.unwrap().deserialize(None) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if record.num == 0 {
                break;
            }
            enemies.push(record);
        }

        RawCSVData {
            header: csv_head,
            line2: csv_line_2,
            enemies,
        }

        // /// temp
        // pub fn read_csv_file(file_name: &str) {
        //     Stage::read_stage_csv(get_decommented_file_reader(file_name).unwrap());
        //     // check all stage files ig
        //     // Encounters: check the head, if needs to be nexted then next it
        //     // do split(',').next()
        //     // if matches string version of target then do serde
        //     // if is "0" then break
        //     // Could make a tester that checks Ms. Sign with the idiomatic and the
        //     // efficient way of doing it.
        //     // Would need to benchmark it though.
        //     // ByteRecord::from(thing.split().collect())
        //     // Could even do just checking id, mag, amag
        // }
    }

    /// Get `map_id` to use in map_option and stage_option.
    fn get_map_id(&self) -> u32 {
        if self.meta.type_num == 3 && [15, 16].contains(&self.meta.map_num) {
            // CotC outbreaks have 22_000 as their map id
            22_000 + self.meta.map_num
        } else {
            self.meta.type_num * 1000 + self.meta.map_num
        }
        // this is definitely very fragile
    }

    /// Get MapStageData data if it exists.
    pub fn get_map_stage_data(&self) -> Option<StageDataCSV> {
        GameMap::get_stage_data(&self.meta)
    }

    /// Get Map_option data if it exists.
    pub fn get_map_option_data(&self) -> Option<MapOptionCSV> {
        let map_id = self.get_map_id();
        MAP_OPTION.get_map(map_id)
    }

    /// Get Stage_option data if it exists.
    pub fn get_stage_option_data(&self) -> Option<impl Iterator<Item = &StageOptionCSV>> {
        let map_id = self.get_map_id();
        STAGE_OPTION.get_stage(map_id, self.meta.stage_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_handler::{get_file_location, FileLocation::GameData};
    use regex::Regex;

    #[test]
    #[ignore]
    fn get_all() {
        let stage_file_re = Regex::new(r"^stage.*?\d{2}\.csv$").unwrap();
        for f in std::fs::read_dir(get_file_location(GameData).join("DataLocal")).unwrap() {
            let file_name = f.unwrap().file_name().into_string().unwrap();
            if !stage_file_re.is_match(&file_name) {
                continue;
            };
            StageData::new(&file_name);
        }
    }
}
