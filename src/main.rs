use rust_wiki::data::{
    map::map_option::MAP_OPTION,
    stage::{parsed::stage::Stage, stage_metadata::StageMeta, stage_option::STAGE_OPTION},
};

// fn benchmark_stage_type(){
//     let start = Instant::now();
//     const iterations: usize = 1000;
//     for i in 0..iterations{
//         for j in 0..iterations{
//             StageType::new(&format!("stageRT{i:03}_{j:02}.csv"));
//         }
//     }
//     println!("{}", start.elapsed().as_secs_f64())
// }

// Look into clap
fn main() {
    println!("{:?}", StageMeta::new("sol 0 0").unwrap());
    println!("{:?}", StageMeta::new("ex 0 0").unwrap());

    do_stuff();
}

/// temp function
fn do_stuff() {
    println!("{:?}", MAP_OPTION.get_map(0));
    println!("{:?}", STAGE_OPTION.get_map(0));
    println!("{:?}", Stage::new("n 0 0").unwrap());
}
