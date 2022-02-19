use ansi_term::Colour::Red;
use std::error::Error;
use crate::piece::Piece;
use crate::game_board::GameBoard;
use std::io;
use crate::directions::Directions;
use simple_ansi::cursor;
use simple_ansi::erase::line;

pub mod game_board;
pub mod piece;
pub mod directions;
pub fn run(pretty: bool) -> String{
	let mut sequence = String::new();
	let mut board = GameBoard::new();
	println!("Game time");
	println!("{}",board);
	let winner = loop{
		match loop{
			match player_turn(&mut board, Piece::Red, &mut sequence, &pretty){
				Ok(true)=>break true,
				Ok(false)=>break false,
				Err(_)=>{
					line();
					println!("{}",Red.paint("Not a column!"));
					println!("{}",board);
					continue;
			},
			}
		}{
			true=>break Piece::Red,
			false=>(),
		}
		match loop{
			match player_turn(&mut board, Piece::Blue, &mut sequence, &pretty){
				Ok(true)=>break true,
				Ok(false)=>break false,
				Err(_)=>{
					line();
					println!("{}",Red.paint("Not a column!"));
					println!("{}",board);
					continue;
			},
			}
		}{
			true=>break Piece::Blue,
			false=>(),
		}
		

	};
	line();
	println!("{} won!",winner);
	sequence
}
fn has_connect_4(board: &GameBoard, x: usize, y: usize, color: Piece, pretty: &bool)->bool{
	let mut directions = Directions::new();
	for i in 1..4 {
		directions.check_directions(x,y,i,board,&color);
		if !*pretty {directions.print()};
		if directions.check_if_any_4(){
			return true;
		}
		if !directions.check_if_any_true(){
			return false;
		}
	}
	return true;
}

fn player_turn(board: &mut GameBoard, piece: Piece, sequence: &mut String, pretty: &bool)->Result<bool,Box<dyn Error>>{
	println!("{} turn",piece);
	let mut play = String::new();
	io::stdin()
		.read_line(&mut play)?;
	if *pretty{
		cursor::move_up(9);
		cursor::move_to_column(0);
	}
	let x: u32 = (play.trim().parse::<u32>()?)-1;
	let y = board.place(piece,x.try_into().unwrap(),&pretty)?;
	println!("{}",board);
	sequence.push_str(&(x+1).to_string());
	if has_connect_4(& board,x.try_into().unwrap(),y.try_into().unwrap(),piece,&pretty){
		return Ok(true);
	}
	Ok(false)
}
