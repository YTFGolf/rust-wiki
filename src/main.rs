use rust_wiki::stage::stage_type::StageType;

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
    println!("{:?}", StageType::new("sol 0 0").unwrap());
    println!("{:?}", StageType::new("ex 0 0").unwrap());
}
