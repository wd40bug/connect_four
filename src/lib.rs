use crate::piece::Piece;
use crate::game_board::GameBoard;
use std::io;
use crate::directions::Directions;
use simple_ansi::cursor;
use simple_ansi::erase::line;

pub mod game_board;
pub mod piece;
pub mod directions;
pub fn run(){
	let mut board = GameBoard::new();
	println!("Game time");
	println!("{}",board);
	let winner = loop{
		if player_turn(&mut board, Piece::Red){
			break Piece::Red;
		}

		if player_turn(&mut board, Piece::Blue){
			break Piece::Blue;
		}

	};
	line();
	println!("{} won!",winner);
}
fn has_connect_4(board: &GameBoard, x: usize, y: usize, color: Piece)->bool{
	let mut directions = Directions::new();
	for i in 1..4 {
		directions.check_directions(x,y,i,board,&color);
		if !directions.check_if_any_true(){
			return false;
		}
	}
	return true;
}

fn player_turn(board: &mut GameBoard, piece: Piece)->bool{
	println!("{} turn",piece);
	let mut play = String::new();
	io::stdin()
		.read_line(&mut play)
		.unwrap();
	cursor::move_up(9);
	cursor::move_to_column(0);
	let x: u32 = (play.trim().parse::<u32>().unwrap())-1;
	let y = board.place(piece,x.try_into().unwrap());
	println!("{}",board);
	if has_connect_4(& board,x.try_into().unwrap(),y.try_into().unwrap(),piece){
		return true;
	}
	false
}
