use std::{fs, time::SystemTime};
use ansi_term::Color::Yellow;
use crate::{solver::Solver, ai_stuff::Position};
#[test]
fn test_scoring(){
    // simple_logger::init().unwrap();
    simple_logger::SimpleLogger::new().with_colors(true).init().unwrap();
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
                log::info!("{}: {} nodes:{} {}",line.to_string().split(" ").collect::<Vec<_>>()[0],score,solver.node_count,elapsed.as_micros());
                assert_eq!(score, line.to_string().split(" ").collect::<Vec<_>>()[1].parse().unwrap());
            }
        }
    }
}