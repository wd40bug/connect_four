use std::{fs, io::Lines, time::SystemTime};

use connect_four::{run, solver::Solver, ai_stuff::Position};


fn main(){
    let mut solver = Solver{ node_count: 0 };
    let contents = fs::read_to_string("src/tests/Test_L2_R1").unwrap();
    for line in contents.lines(){
        let mut pos = Position::new();
        if !pos.set_up(line.to_string()) {
            println!("you done messed up");
        } else{
            let now = SystemTime::now();
            let score = solver.solve(&pos);
            if let Ok(elapsed) = now.elapsed(){
                println!("{line}: {score} nodes:{} {}",solver.node_count,elapsed.as_micros());
            }
        }
    }
    // println!("{:?}",std::env::args().nth(1));
    // let pretty = if let Some(arg) = std::env::args().nth(1){
    //     if arg.eq("debug"){
    //         false
    //     } else if arg.eq("pretty"){
    //         true
    //     } else{
    //         true
    //     }
    // }else if !std::env::var("PRETTY_OFF").is_err(){
    //     false
    // }else{
    //     true
    // };
    // println!("sequence: {}",run(pretty));
}