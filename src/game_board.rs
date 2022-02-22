use crate::clean;
use crate::piece::Piece;

use core::time;
use std::fmt::Display;

use std::fmt::Formatter;
use std::thread;


use arr_macro::arr;


#[derive(Clone)]
pub struct GameBoard{
    board: [[Piece;6];7],
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
    pub fn check_tie(&self) -> bool{
        for i in 0..7{
            if self.board[i][5].variant_eq(&Piece::None){
                return false;
            }
        }
        return true;
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
            clean(7);
            thread::sleep(time::Duration::from_millis(70));
        }
    }
    pub fn get(&self, x:usize, y:usize)->Result<&Piece,String>{
        if (0..7).contains(&x)&&(0..6).contains(&y){
            return Ok(&self.board[x][y]);
        }
        Err("Value not in board".to_string())
    }
    pub fn undo(&mut self, x:usize){
        for i in 0..6 {
            match self.board[x][i]{
                Piece::None=>{self.board[x][i-1] = Piece::None; break;},
                _=>(),

            }
        }
    }
    pub fn dump(&mut self){
        for _c in 0..5{
            for i in 0..5{
                for j in 0..7{
                    self.board[j][i] = self.board[j][i+1];
                }
            }
            for i in 0..7{
               self.board[i][5] = Piece::None;
            }
            println!("{}",self);
            clean(7);
            thread::sleep(time::Duration::from_millis(300));
        } 
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
            result.push_str("\n");
        }
        result.push_str(" 01 02 03 04 05 06 07 ");
        write!(f,"{}",result)
    }
}