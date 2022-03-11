pub fn bits(){}
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