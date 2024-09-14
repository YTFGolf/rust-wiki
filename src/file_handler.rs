//! Contains functions to read data files.
use shellexpand;
use std::fs;

/// a
pub fn do_stuff() {
    let file_name = shellexpand::tilde("~/.bash_history").to_string();
    let content = fs::read_to_string(file_name).expect("File name no existo!");
    println!("{content:?}");
}
