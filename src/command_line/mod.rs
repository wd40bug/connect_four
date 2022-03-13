use ansi_term::Color;

use crate::ai_stuff::Position;

pub fn bits(){}
pub fn seq(){}
pub fn u64_to_bit_array(num: u64) -> [u8; 64] {
    let mut num = num;
    let mut result = [0 as u8; 64];
    for i in 0..64 {
        result[i] = (num & 1) as u8;
        num >>= 1;
    }
    result
}
pub fn print_bits(bits: [u8; 64]) {
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
pub fn print_seq(seq: &String) {
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