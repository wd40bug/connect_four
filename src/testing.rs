use std::{fs, time::SystemTime};
use ansi_term::Color::Yellow;
use crate::{solver::Solver, ai_stuff::Position};
#[test]
fn test_scoring(){
    let mut nodes: u64 = 0;
    let mut tests: u64 = 0;
    let mut total_time:u128 = 0;
    pretty_env_logger::init();
    let mut solver = Solver{ node_count: 0 };
    let contents = fs::read_to_string("src/tests/Test_L3_R1").unwrap();
    for line in contents.lines(){
        let mut pos = Position::new();
        if !pos.set_up(line.to_string().split(" ").collect::<Vec<_>>()[0].to_string()) {
            log::warn!("{}",Yellow.paint("you done messed up"));
        } else{
            let now = SystemTime::now();
            let score = solver.solve(&pos);
            if let Ok(elapsed) = now.elapsed(){
                nodes+=solver.node_count;
                tests+=1;
                let elapsed = elapsed.as_micros();
                total_time+=elapsed;
                log::info!("sequence: {0: <50} score: {1: <10} nodes:{2: <10} time: {3: <10}",line.to_string().split(" ").collect::<Vec<_>>()[0],score,solver.node_count,elapsed);
                assert_eq!(score, line.to_string().split(" ").collect::<Vec<_>>()[1].parse().unwrap());
            }
        }
    }
    log::info!("average node count: {}, average time: {}, total time: {}, total tests: {}",nodes/tests,total_time/tests as u128,total_time,tests);
}