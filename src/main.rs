use std::{time::SystemTime, u64};

use ansi_term::Color;
use connect_four::{
    ai_stuff::Position, run, solver::Solver, transposition_table::TranspositionTable, command_line::{bits, u64_to_bit_array, print_bits},
};
use log::LevelFilter;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        match &arg[..] {
            "seq" => {
                if let Some(_) = std::env::args().nth(3){
                    let mut seq = if let Some(seq) = std::env::args().nth(2) {
                        seq
                    } else {
                        let mut foo = String::new();
                        std::io::stdin().read_line(&mut foo).unwrap();
                        foo.trim().to_string()
                    };
                    simple_logging::log_to_stderr(LevelFilter::Debug);
                    let mut solver = Solver {
                        node_count: 0,
                        column_order: [3, 2, 4, 1, 5, 0, 6],
                        transposition_table: TranspositionTable::new(8388593),
                    };
                    let mut pos = Position::new();
                    if !pos.set_up(seq.clone()) {
                        log::warn!("You have messed up");
                    } else {
                        let current_player = pos.moves%2;
                        loop {
                            seq = String::new();
                            let now = pos.moves%2;
                            let score = if now == current_player{
                                solver.solve(&pos, &None)
                            } else{
                                -solver.solve(&pos, &None)
                            };
                            log::debug!("Position: {}, Score: {}", ansi_term::Color::Cyan.paint(&pos.seq).to_string(), score);
                            std::io::stdin().read_line(&mut seq).unwrap();
                            seq = seq.trim().to_string();
                            match &seq[..]{
                                ".."=>{
                                    let mut foo = pos.seq.clone();
                                    foo.pop();
                                    pos = Position::new();
                                    pos.set_up(foo);
                                },
                                "ls"=>{
                                    for x in 0..7{
                                        if pos.possible_non_loosing_moves() & Position::column_mask(x)!=0{
                                            let mut pos2 = pos.clone();
                                            pos2.play_col(x);
                                            log::debug!("Score: {}, Play: {}",-solver.solve(&pos2, &None), x);
                                        }
                                    }
                                }
                                _=>{
                                    if let Ok(col) = seq.parse(){
                                        pos.play(col);
                                    }
                                },
                            }
                        }
                    }
                } else{
                    let seq = if let Some(seq) = std::env::args().nth(2) {
                        seq
                    } else {
                        let mut foo = String::new();
                        std::io::stdin().read_line(&mut foo).unwrap();
                        foo.trim().to_string()
                    };
                    simple_logging::log_to_stderr(LevelFilter::Debug);
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
                        print_seq(&seq);
                        log::info!("position: {}",pos.seq);
                        let score = solver.solve(&pos, &None);
                        log::info!("seq: {}, pos: {}, score: {}, time: {}\u{00B5}, nodes: {}, time per node: {}",&seq,pos.current_position,score,now.elapsed().unwrap().as_millis(),solver.node_count, solver.node_count as u128/now.elapsed().unwrap().as_micros());
                    }
                }
            }
            "play" => {
                let pretty = if let Some(play_arg) = std::env::args().nth(2) {
                    if play_arg.eq("pretty") {
                        true
                    } else if play_arg.eq("debug") {
                        false
                    } else {
                        true
                    }
                } else if !std::env::var("PRETTY_OFF").is_err() {
                    false
                } else {
                    true
                };
                println!("sequence: {}", run(pretty));
            }
            "bits" => {
                bits();
            },
            _ => {
                println!("argument not recognized");
            }
        }
    } else {
        let pretty = if !std::env::var("PRETTY_OFF").is_err() {
            false
        } else {
            true
        };
        println!("sequence: {}", run(pretty));
    };
}
fn print_seq(seq: &String) {
    let mut pos = Position::new();
    pos.set_up((*seq).clone());
    let pos_bits = u64_to_bit_array(pos.current_position);
    let mask_bits = u64_to_bit_array(pos.mask);
    let bottom_bits = u64_to_bit_array(Position::BOTTOM_MASK);
    let key_bits = u64_to_bit_array(pos.key());
    let board_bits = u64_to_bit_array(Position::BOARD_MASK);
    let winning_bits = u64_to_bit_array(Position::compute_winning_position(pos.current_position, pos.mask));
    let possible_bits = u64_to_bit_array(pos.possible());
    println!("");
    println!("Current position: {}", Color::Green.paint(pos.current_position.to_string()));
    print_bits(pos_bits);
    println!("Mask:             {}", Color::Green.paint(pos.mask.to_string()));
    print_bits(mask_bits);
    println!("Bottom Mask:      {}", Color::Green.paint(Position::BOTTOM_MASK.to_string()));
    print_bits(bottom_bits);
    println!("Key:              {}", Color::Green.paint(pos.key().to_string()));
    print_bits(key_bits);
    println!("Board Mask:       {}", Color::Green.paint(Position::BOARD_MASK.to_string()));
    print_bits(board_bits);
    println!("Possible:         {}", Color::Green.paint(pos.possible().to_string()));
    print_bits(possible_bits);
    println!("Winning Position: {}", Color::Green.paint(Position::compute_winning_position(pos.current_position, pos.mask).to_string()));
    print_bits(winning_bits);
    println!("Opponent Winning: {}", Color::Green.paint(pos.opponent_winning_position().to_string()));
    print_bits(u64_to_bit_array(pos.opponent_winning_position()));
    println!("Can win next:     {}", Color::Green.paint(pos.can_win_next().to_string()));
    println!("Forced Move:      {}", Color::Green.paint(pos.forced_moves().to_string()));
    print_bits(u64_to_bit_array(pos.forced_moves()));
    println!("Not Loosing:      {}", Color::Green.paint(pos.possible_non_loosing_moves().to_string()));
    print_bits(u64_to_bit_array(pos.possible_non_loosing_moves()));
}
