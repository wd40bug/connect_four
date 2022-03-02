use std::time::SystemTime;

use connect_four::{run, solver::Solver, ai_stuff::Position};


fn main(){
    if let Some(arg) = std::env::args().nth(1){
        if arg.eq("seq"){
            let seq = if let Some(seq) = std::env::args().nth(2){
                seq
            } else{
                let mut foo = String::new();
                std::io::stdin().read_line(&mut foo).unwrap();
                foo.trim().to_string()
            };
            pretty_env_logger::init();
            let mut solver = Solver{node_count: 0, column_order: [3,2,4,1,5,0,6]};
            let mut pos = Position::new();
            if !pos.set_up(seq.clone()){
                log::warn!("You have messed up");
            } else{
                let now = SystemTime::now();
                let score = solver.solve(&pos);
                log::info!("seq: {}, score: {}, time: {}\u{00B5}, nodes: {}, time per node: {}",&seq,score,now.elapsed().unwrap().as_millis(),solver.node_count, solver.node_count as u128/now.elapsed().unwrap().as_millis());
            }
        } else if arg.eq("play"){
            let pretty = if let Some(play_arg) = std::env::args().nth(2){
                if play_arg.eq("pretty"){
                    true
                }else if play_arg.eq("debug"){
                    false
                } else{
                    true
                }
            }else if !std::env::var("PRETTY_OFF").is_err(){
                false
            } else{
                true
            };
            println!("sequence: {}",run(pretty));
        }
    } else{
        let pretty = if !std::env::var("PRETTY_OFF").is_err(){
            false
        } else{true};
        println!("sequence: {}",run(pretty));
    };
}