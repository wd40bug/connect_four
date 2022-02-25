use std::{fmt::Display};
use ansi_term::Colour::{Red,Cyan,White};

#[derive(Clone)]
pub struct Position{
    pub board: BoardWrapper,
    pub height: [u32;7],
    pub moves: u32,
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
    pub fn can_play(&self, col: usize)->bool{
        self.height[col] < 6
    }
    pub fn play(&mut self, col: usize){
        self.board.0[col][self.height[col] as usize] = 1+self.moves%2;
        self.height[col]+=1;
        self.moves+=1;
    }
    pub fn is_winning_move(&self, col: usize)->bool{
        let current_player = 1+self.moves%2;
        if self.height[col] >=3 
            && self.board.0[col][self.height[col] as usize-1] == current_player
            && self.board.0[col][self.height[col] as usize-2] == current_player
            && self.board.0[col][self.height[col] as usize-3] == current_player 
            {return true;}
        for dy in -1i32..=1{
            let mut nb = 0;
            for dx in (-1i32..=1).step_by(2){
                let mut x = col as i32+dx;
                let mut y = self.height[col]as i32+dx*dy;
                while x >= 0 && x < y && y >= 0 && y < 6 && self.board.0[x as usize][y as usize] == current_player{
                    x+=dx;
                    y+=dy*dx;
                    nb+=1;
                }
            }
            if nb >= 3{
                return true;
            }
        }
        false
    }
    pub fn set_up(&mut self, seq: String) -> bool{
        println!("{}",seq);
        for i in 0..seq.len(){
            let col:i32 = seq.chars().nth(i).unwrap().to_digit(10).unwrap() as i32 -1;
            if col < 0 || col >= 7 || !self.can_play(col as usize) || self.is_winning_move(col as usize){
                return false
            } else{
                self.play(col as usize);
            }
        }
        log::info!("set_up complete for {}",seq);
        log::info!("\n\n{}",self.board);
        true
    }
    pub fn new()->Position{
        Position{
            board: BoardWrapper([[0;6];7]),
            height: [0;7],
            moves: 0,
        }
    }

}