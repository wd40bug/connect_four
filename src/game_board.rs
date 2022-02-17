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
    pub fn place(&mut self, piece: Piece,x:usize){
        for i in 0..6 {
            match self.board[x][i]{
                Piece::None => {
                    self.board[x][i] = piece;
                    break;
                },
                _=>continue,
            }
        }
    }
}
impl Display for GameBoard{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let mut result = String::new();
        for i in (0..6).rev() {
            for column in &self.board{
                result.push_str(&column[i].to_string());
            }
            result.push_str("\n")
        }
        write!(f,"{}",result)
    }
}