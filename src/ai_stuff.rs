use std::{char};

const HEIGHT: i32 = 6;
const WIDTH: i32 =7;
#[derive(Clone)]
pub struct Position{
    pub mask: u64,
    pub current_position: u64,
    pub moves: u32,
    pub seq: String,
}
impl Position{
    pub const BOTTOM_MASK:u64 = Self::bottom(WIDTH, HEIGHT);
    pub const BOARD_MASK: u64 = Self::BOTTOM_MASK * ((1<<HEIGHT)-1);
    pub const fn bottom(width: i32, height: i32) -> u64{
        return if width == 0{
            0
        } else{
            Self::bottom(width-1, height) | 1 << (width-1)*(height+1)
        }
    }
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
        1 << ((HEIGHT-1)+col as i32*(HEIGHT+1))
    }
    fn bottom_mask_col(col: u64) -> u64{
        1 << col*(HEIGHT+1) as u64
    }
    fn column_mask(col: u64) -> u64{
        ((1<<HEIGHT)-1)<<col as i32*(HEIGHT+1)
    }
    pub fn can_play(&self, col: usize)->bool{
        self.mask & Self::top_mask(col as u64) == 0
    }
    pub fn play(&mut self, col: usize){
        self.seq.push(char::from_digit(col as u32, 10).unwrap());
        self.current_position ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask_col(col as u64);
        self.moves+=1;
    }
    pub fn is_winning_move(&self, col: usize)->bool{
        let mut pos: u64 = self.current_position;
        pos |= (self.mask + Self::bottom_mask_col(col as u64)) & Self::column_mask(col as u64);
        Self::alignment(pos)
    }
    pub fn set_up(&mut self, seq: String) -> bool{
        for (j,i) in seq.chars().enumerate(){
            let col:isize = i.to_digit(10).unwrap() as isize - 1;
            if col<0||col>=WIDTH as isize||!self.can_play(col as usize)||self.is_winning_move(col as usize){
                log::error!("set up failed at character{} at index {}",i,j);
                return false;
            }
            self.play(col as usize);
        }
        return true;
    }
    pub fn key(&self) -> u64{
        self.current_position+self.mask
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