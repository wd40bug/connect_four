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
                let seq = if let Some(seq) = std::env::args().nth(2) {
                    seq
                } else {
                    let mut foo = String::new();
                    std::io::stdin().read_line(&mut foo).unwrap();
                    foo.trim().to_string()
                };
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
    println!("");
    println!("{}", Color::Green.paint(pos.current_position.to_string()));
    print_bits(pos_bits);
    println!("");
    println!("{}", Color::Green.paint(pos.mask.to_string()));
    print_bits(mask_bits);
    println!("{}", Color::Green.paint(Position::BOTTOM_MASK.to_string()));
    print_bits(bottom_bits);
    println!("{}", Color::Green.paint(pos.key().to_string()));
    print_bits(key_bits);
    println!("{}", Color::Green.paint(Position::BOARD_MASK.to_string()));
    print_bits(board_bits);
}
