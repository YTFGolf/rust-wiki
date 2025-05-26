//! Utility functions for dealing with stages.

use super::super::version::Version;
use crate::regex_handler::static_regex;

/// Get a list of all stage data files in the game.
pub fn get_stage_files(version: &Version) -> impl Iterator<Item = String> {
    let stage_file_re = static_regex(r"^stage.*?\d{2}\.csv$");
    let dir = &version.get_file_path("DataLocal");

    let files = std::fs::read_dir(dir).unwrap();

    files.filter_map(move |f| {
        let file_name = f.unwrap().file_name().into_string().unwrap();
        // needs to be converted to string so regex works

        if stage_file_re.is_match(&file_name) {
            Some(file_name)
        } else {
            None
        }
    })
}
