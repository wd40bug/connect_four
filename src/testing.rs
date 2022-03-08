use std::{fs::{self, File}, time::SystemTime};
use ansi_term::Color::Yellow;
use chrono::Local;
use crate::{solver::Solver, ai_stuff::Position, transposition_table::TranspositionTable};
use indicatif::{ProgressBar, ProgressStyle};
use log::LevelFilter;
#[test]
fn test_scoring(){
    let mut nodes: u64 = 0;
    let mut tests: u64 = 0;
    let mut total_time:u128 = 0;
    let date_time = Local::now().timestamp();
    File::create(format!("logs/{}.log",date_time)).unwrap();
    simple_logging::log_to_file(format!("logs/{}.log",date_time), LevelFilter::Info).unwrap();
    let mut solver = Solver{ node_count: 0, column_order: [3,2,4,1,5,0,6], transposition_table: TranspositionTable::new(8388593) };
    let contents = fs::read_to_string("src/tests/Test_L2_R1").unwrap();
    let pb = ProgressBar::new(1000);
    pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:40.cyan/blue} {msg:<50} {pos}/{len} ({eta_precise})")
        .progress_chars("#=*")
    );
    for line in contents.lines(){
        let mut pos = Position::new();
        if !pos.set_up(line.to_string().split(" ").collect::<Vec<_>>()[0].to_string()) {
            log::warn!("{}",Yellow.paint("you done messed up"));
        } else{
            let now = SystemTime::now();
            pb.set_message(pos.seq.clone());
            let score = solver.solve(&pos);
            if let Ok(elapsed) = now.elapsed(){
                pb.inc(1);
                nodes+=solver.node_count;
                tests+=1;
                let elapsed = elapsed.as_micros();
                total_time+=elapsed;
                log::info!("sequence: {0: <50} score: {1: <10} nodes:{2: <10} time: {3}\u{00B5}s",line.to_string().split(" ").collect::<Vec<_>>()[0],score,solver.node_count,elapsed);
                assert_eq!(score, line.to_string().split(" ").collect::<Vec<_>>()[1].parse().unwrap());
            }
        }
    }
    log::info!("average node count: {}, total_node_count: {}, average time: {}\u{00B5}s, total time: {}\u{00B5}s, total tests: {}",nodes/tests,nodes,total_time/tests as u128,total_time,tests);
}
