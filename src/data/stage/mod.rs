//! Module that deals with getting information about a stage.

pub mod parsed;
pub mod raw;
use super::version::Version;
use raw::stage_data::StageData;
use regex::Regex;

/// Get an iterator over all stages in the version.
pub fn get_stages(version: &Version) -> impl Iterator<Item = StageData<'_>> {
    let stage_file_re = Regex::new(r"^stage.*?\d{2}\.csv$").unwrap();
    let dir = &version.get_file_path("DataLocal");

    let files = std::fs::read_dir(dir).unwrap();
    let stages = files.filter_map(move |f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();
        if !stage_file_re.is_match(&file_name) {
            return None;
        };

        let stage = StageData::new(&file_name, version).unwrap();
        Some(stage)
    });

    stages
}
