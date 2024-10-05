use rust_wiki::{
    data::stage::parsed::stage::Stage, wiki_files::update_wiki_files,
    wikitext::stage_info::get_stage_info,
};
use std::io::{self, Write};

// Look into clap
fn main() {
    if false {
        // if true {
        update_wiki_files();
    }

    print!("Input file selector: ");
    io::stdout().flush().unwrap();
    let selector = io::stdin().lines().next().unwrap().unwrap();
    println!("{selector:?}");

    println!("{}", get_stage_info(&Stage::new(&selector).unwrap()))
}
