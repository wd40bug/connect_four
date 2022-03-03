use std::{fmt::Display, char};
use ansi_term::Colour::{Red,Cyan,White};

#[derive(Clone)]
pub struct Position{
    mask: u64,
    current_position: u64,
    pub moves: u32,
    pub seq: String,
}
#[derive(Clone)]
pub struct BoardWrapper([[u32; 6];7]);
impl Display for BoardWrapper{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for i in (0..6).rev(){
            for j in 0..7{
                result.push_str(&match self.0[j][i]{
                    1=>Red.paint("1"),
                    2=>Cyan.paint("2"),
                    _=>White.paint("0"), 
                }.to_string());
            }
            result.push('\n');
        }
        result.pop();
        write!(f,"{}", result)
    }
}
impl Position{
    fn alignment(pos: u64)->bool{
        let mut m: u64 = pos & (pos >> 7);
        if m & (m>>14)!=0 {return true;}

        m = pos & (pos>>6);
        if m & (m>>12)!=0 {return  true;}

        m = pos & (pos >> 8);
        if m & (m >> 16)!=0 {return true;}

        m = pos & (pos >> 1);
        if m & (m >> 2)!=0 {return true;}

        return false;
    }
    fn top_mask(col:u64) -> u64{
        (1 << 5) << col*7
    }
    fn bottom_mask(col: u64) -> u64{
        1 << col*7
    }
    fn column_mask(col: u64) -> u64{
        ((1<<6)-1)<<col*7
    }
    pub fn can_play(&self, col: usize)->bool{
        self.mask & Self::top_mask(col as u64) == 0
    }
    pub fn play(&mut self, col: usize){
        self.seq.push(char::from_digit(col as u32, 10).unwrap());
        self.current_position ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask(col as u64);
        self.moves+=1;
    }
    pub fn is_winning_move(&self, col: usize)->bool{
        let mut pos: u64 = self.current_position;
        pos |= (self.mask + Self::bottom_mask(col as u64)) & Self::column_mask(col as u64);
        Self::alignment(pos)
    }
    pub fn set_up(&mut self, seq: String) -> bool{
        for (j,i) in seq.chars().enumerate(){
            let col:isize = i.to_digit(10).unwrap() as isize - 1;
            if col<0||col>=7||!self.can_play(col as usize)||self.is_winning_move(col as usize){
                log::error!("set up failed at character{} at index {}",i,j);
                return false;
            }
            self.play(col as usize);
        }
        return true;
    }
    pub fn new()->Position{
        Position{
            current_position: 0,
            mask: 0,
            moves: 0,
            seq: String::new(),
        }
    }

}