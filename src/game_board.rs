use crate::piece::Piece;

use core::time;
use std::fmt::Display;

use std::fmt::Formatter;
use std::thread;


use arr_macro::arr;
use simple_ansi::cursor;

#[derive(Clone)]
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
    pub fn place(&mut self, piece: Piece,x:usize, pretty: &bool) -> Result<usize,String>{
        if !(0..7).contains(&x){
            return Err("Not a valid column".to_string());
        }
        for i in 0..6 {
            match self.board[x][i]{
                Piece::None => {
                    if *pretty {self.fall_to(x, i, piece);}
                    self.board[x][i] = piece;
                    return Ok(i);
                },
                _=>continue,
            }
        }
        return Err("No room in column".to_string());
    }
    fn with(&self,x:usize,y:usize,piece:Piece)->GameBoard{
        let mut self_clone = self.clone();
        self_clone.board[x][y] = piece;
        self_clone
    }
    fn fall_to(&self, x:usize, y:usize, piece: Piece){
        for i in (y..6).rev(){
            println!("{}",self.with(x, i, piece));
            cursor::move_up(7);
	        cursor::move_to_column(0);
            thread::sleep(time::Duration::from_millis(100))
        }
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
        result.push_str(" 01 02 03 04 05 06 07 ");
        write!(f,"{}",result)
    }
}