use rust_wiki::{
    data::stage::{parsed::stage::Stage, stage_metadata::StageMeta},
    wiki_files::update_wiki_files,
    wikitext::stage_info::do_stuff,
};

// Look into clap
fn main() {
    println!("{:?}", StageMeta::new("sol 0 0").unwrap());
    println!("{:?}", StageMeta::new("ex 0 0").unwrap());
    println!("{:?}", Stage::new("a 68 0").unwrap());
    if false {
        // if true {
        update_wiki_files();
    }

    do_stuff();
}
