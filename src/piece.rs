
use std::fmt::Formatter;
use std::fmt::Display;
use ansi_term::Style;

use ansi_term::Colour::Red;

use ansi_term::Colour::Blue;

use ansi_term::Colour::White;

#[derive(Copy,Clone)]
pub enum Piece{
    Red,
    Blue,
    None,
}
impl Piece{
    pub fn to_string(&self)->String{
        match self{
            Piece::Red=>Style::new().on(Red).paint("O"),
            Piece::Blue=>Style::new().on(Blue).paint("O"),
            Piece::None=>White.paint(" "),
        }.to_string()
    }
    pub fn variant_eq(&self, other: &Piece)->bool{
        std::mem::discriminant(self)==std::mem::discriminant(other)
    }
}
impl Display for Piece{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let result = match self{
            Piece::Red=>"Red",
            Piece::Blue=>"Blue",
            Piece::None=>"None",
        };
        write!(f,"{}",result)
    }
}