use std::time::SystemTime;

use log::LevelFilter;

use crate::{transposition_table::TranspositionTable, ai_stuff::Position, solver::Solver};

pub fn seq(seq: String){
    
    simple_logging::log_to_stderr(LevelFilter::Info);
    let mut solver = Solver {
        node_count: 0,
        column_order: [3, 2, 4, 1, 5, 0, 6],
        transposition_table: TranspositionTable::new(8388593),
    };
    let mut pos = Position::new();
    if !pos.set_up(seq.clone()) {
        log::warn!("You have messed up");
    } else {
        let now = SystemTime::now();
        let score = solver.solve(&pos);
        log::info!("seq: {}, pos: {}, score: {}, time: {}\u{00B5}, nodes: {}, time per node: {}",&seq,pos.current_position,score,now.elapsed().unwrap().as_millis(),solver.node_count, solver.node_count as u128/now.elapsed().unwrap().as_micros());
    }
}