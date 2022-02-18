use crate::piece::Piece;

use std::fmt::Display;

use std::fmt::Formatter;


use arr_macro::arr;

pub struct GameBoard{
    board: [[Piece;6];7]
}
impl GameBoard{
    pub fn new()->GameBoard{
        fn empty_row()->[Piece;6]{
            arr![Piece::None;6]
        }
        GameBoard{
            board: arr![empty_row();7],
        }   
    }
    pub fn place(&mut self, piece: Piece,x:usize) -> usize{
        for i in 0..6 {
            match self.board[x][i]{
                Piece::None => {
                    self.board[x][i] = piece;
                    return i;
                },
                _=>continue,
            }
        }
        return 100;
    }
    pub fn get(&self, x:usize, y:usize)->Result<&Piece,String>{
        if (0..7).contains(&x)&&(0..6).contains(&y){
            return Ok(&self.board[x][y]);
        }
        Err("Value not in board".to_string())
    }
    pub fn check_in(&self, x:usize, y:usize) -> bool{
        if let Ok(_foo) = self.get(x,y){
            return true;
        } else{
            return false;
        }
    }
}
impl Display for GameBoard{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut result = String::new();
        for i in (0..6).rev() {
            result.push_str("|");
            for column in &self.board{
                result.push_str(&column[i].to_string());
                result.push_str("|");
            }
            result.push_str("\n")
        }
        write!(f,"{}",result)
    }
}