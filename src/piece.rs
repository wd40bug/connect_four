use ansi_term::Style;

use ansi_term::Colour::Red;

use ansi_term::Colour::Blue;

use ansi_term::Colour::White;
pub enum Piece{
    Red,
    Blue,
    None,
}
impl Piece{
    pub fn to_string(&self)->String{
        match self{
            Piece::Red=>Style::new().on(Red).paint("[ ]"),
            Piece::Blue=>Style::new().on(Blue).paint("[ ]"),
            Piece::None=>White.paint("[ ]"),
        }.to_string()
    }
}