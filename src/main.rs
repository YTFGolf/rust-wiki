use rust_wiki::{data::stage::stage_metadata::StageMeta, wikitext::stage_info::do_stuff};

// Look into clap
fn main() {
    println!("{:?}", StageMeta::new("sol 0 0").unwrap());
    println!("{:?}", StageMeta::new("ex 0 0").unwrap());

    do_stuff();
}
