use std::{time::SystemTime, u64};

use ansi_term::Color;
use connect_four::{
    ai_stuff::Position, run, solver::Solver, transposition_table::TranspositionTable,
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
                                solver.solve(&pos)
                            } else{
                                -solver.solve(&pos)
                            };
                            log::debug!("Position: {}, Score: {}", ansi_term::Color::Cyan.paint(&pos.seq).to_string(), score);
                            std::io::stdin().read_line(&mut seq).unwrap();
                            seq = seq.trim().to_string();
                            match &seq[..]{
                                ".."=>{
                                    seq.pop();
                                    pos.set_up(seq.clone());
                                },
                                "ls"=>{
                                    for x in 0..7{
                                        if pos.possible_non_loosing_moves() & Position::column_mask(solver.column_order[x] as u64)!=0{
                                            let mut pos2 = pos.clone();
                                            pos2.play(solver.column_order[x]);
                                            log::debug!("Score: {}, Play: {}",-solver.solve(&pos2), solver.column_order[x]);
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
                        let score = solver.solve(&pos);
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
                if let Some(sub_arg) = std::env::args().nth(2) {
                    if let Ok(num) = sub_arg.parse() {
                        let bits = u64_to_bit_array(num);
                        print_bits(bits);
                    } else {
                        match &sub_arg[..] {
                            "seq" => {
                                if let Some(seq) = std::env::args().nth(3) {
                                    print_seq(&seq);
                                } else {
                                    let mut seq = String::new();
                                    std::io::stdin().read_line(&mut seq).unwrap();
                                    print_seq(&seq);
                                }
                            }
                            "eval" => {
                                let (num1, num2) =
                                    if let Some(sub_sub_sub_arg) = std::env::args().nth(4) {
                                        if let Ok(num1) = sub_sub_sub_arg.trim().parse() {
                                            if let Some(num2_string) = std::env::args().nth(5) {
                                                if let Ok(num2) = num2_string.parse() {
                                                    (num1, num2)
                                                } else {
                                                    println!(
                                                        "must provide a number for second argument"
                                                    );
                                                    std::process::exit(1)
                                                }
                                            } else {
                                                println!("must provide a second argument");
                                                std::process::exit(1)
                                            }
                                        } else {
                                            let mut num1 = String::new();
                                            std::io::stdin().read_line(&mut num1).unwrap();
                                            let mut num2 = String::new();
                                            std::io::stdin().read_line(&mut num2).unwrap();
                                            let num1: u64 = num1.trim().parse().unwrap();
                                            let num2: u64 = num2.trim().parse().unwrap();
                                            (num1, num2)
                                        }
                                    } else {
                                        let mut num1 = String::new();
                                        std::io::stdin().read_line(&mut num1).unwrap();
                                        let mut num2 = String::new();
                                        std::io::stdin().read_line(&mut num2).unwrap();
                                        let num1: u64 = num1.trim().parse().unwrap();
                                        let num2: u64 = num2.trim().parse().unwrap();
                                        (num1, num2)
                                    };
                                if let Some(sub_sub_arg) = std::env::args().nth(3) {
                                    match &sub_sub_arg[..] {
                                        "rs" => println!("{}", num1 >> num2),
                                        "ls" => println!("{}", num1 << num2),
                                        "xor" => println!("{}", num1 ^ num2),
                                        "and" => println!("{}", num1 & num2),
                                        "or" => println!("{}", num1 | num2),
                                        _ => println!("not a valid operation"),
                                    }
                                } else {
                                    println!("need a sub argument")
                                }
                            }
                            _ => println!("not a known sub command of 'bits' "),
                        }
                    }
                } else {
                    let mut num = String::new();
                    std::io::stdin().read_line(&mut num).unwrap();
                    println!("");
                    let num: u64 = num.trim().parse().unwrap();
                    let bits = u64_to_bit_array(num);
                    print_bits(bits);
                }
            }
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
fn u64_to_bit_array(num: u64) -> [u8; 64] {
    let mut num = num;
    let mut result = [0 as u8; 64];
    for i in 0..64 {
        result[i] = (num & 1) as u8;
        num >>= 1;
    }
    result
}
fn print_bits(bits: [u8; 64]) {
    for i in 0..7 {
        for j in 0..7 {
            let x = match bits[(6 - i) + (7 * j)] {
                1 => ansi_term::Color::Red.paint(1.to_string()).to_string(),
                _ => 0.to_string(),
            };
            print!("{}", x);
        }
        println!("");
    }
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
