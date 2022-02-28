use simple_logger;
use std::{fs, time::SystemTime, io};

use connect_four::{solver::Solver, ai_stuff::Position, run};


fn main(){
    simple_logger::init().unwrap();

    // let mut seq = String::new();
    // io::stdin().read_line(&mut seq).unwrap();
    // let mut pos = Position::new();
    // if !pos.set_up(seq.trim().to_string()){
    //     println!("not a valid sequence");
    // } else{

    //     // println!("{}",pos.is_winning_move(6));

    //     let now = SystemTime::now();
    //     let score = solver.solve(&pos);
    //     if let Ok(elapsed) = now.elapsed(){
    //         println!("position: {}, score: {} nodes: {}, time in millis: {}",seq.trim(),score,solver.node_count,elapsed.as_micros());
    //     }
    // }

    // 


    println!("{:?}",std::env::args().nth(1));
    let pretty = if let Some(arg) = std::env::args().nth(1){
        if arg.eq("debug"){
            false
        } else if arg.eq("pretty"){
            true
        } else{
            true
        }
    }else if !std::env::var("PRETTY_OFF").is_err(){
        false
    }else{
        true
    };
    println!("sequence: {}",run(pretty));
}