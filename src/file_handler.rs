//! Contains functions to read data files.
use std::fs;
use crate::config::CONFIG;

/// temp function
pub fn do_stuff() {
    let file_name = "DataLocal/stage.csv";
    let content = fs::read_to_string(CONFIG.data_mines.join(file_name)).expect("File name no existo!");
    println!("{content:?}");
}
