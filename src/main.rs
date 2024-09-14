use rust_wiki::{file_handler::do_stuff, stage::stage_metadata::StageMeta};

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

fn main() {
    println!("{:?}", StageMeta::new("sol 0 0").unwrap());
    println!("{:?}", StageMeta::new("ex 0 0").unwrap());

    do_stuff();
}
