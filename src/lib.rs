use crate::piece::Piece;
use crate::game_board::GameBoard;
use std::io;

pub mod game_board;
pub mod piece;
pub fn run(){
	let mut x = 0;
	loop{
		let mut play = String::new();
		io::stdin()
			.next_line(&mut play)
			.unwrap();
		let x = play.parse
	}
}
pub fn has_connect_4(board: GameBoard, x:i32, y:i32, color: Piece)->bool{
	let mut row = 0;
	for i in y-3..=y+3{
		if let Ok(piece) = board.get(x.try_into().unwrap(),i.try_into().unwrap()){
			match piece{
				Piece::Red=>{
					match color{
						Piece::Red=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				},
				Piece::Blue=>{
					match color{
						Piece::Blue=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				}
				_=>row = 0,
			}
		} else{
			continue;
		}
	}
	row = 0;
	for i in x-3..=x+3 {
		if let Ok(piece) = board.get(x.try_into().unwrap(),i.try_into().unwrap()){
			match piece{
				Piece::Red=>{
					match color{
						Piece::Red=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				},
				Piece::Blue=>{
					match color{
						Piece::Blue=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				}
				_=>row = 0,
			}
		} else{
			continue;
		}
	}
	row = 0;
	for i in -3..=3 {
		if let Ok(piece) = board.get((x+i).try_into().unwrap(),(y+i).try_into().unwrap()){
			match piece{
				Piece::Red=>{
					match color{
						Piece::Red=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				},
				Piece::Blue=>{
					match color{
						Piece::Blue=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				}
				_=>row = 0,
			}
		} else{
			continue;
		}
	}
	row = 0;
	for i in -3..=3 {
		if let Ok(piece) = board.get((x+i).try_into().unwrap(),(y-i).try_into().unwrap()){
			match piece{
				Piece::Red=>{
					match color{
						Piece::Red=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				},
				Piece::Blue=>{
					match color{
						Piece::Blue=>{
							row+=1;
							if row == 4{
								return true;
							}
						},
						_=>row=0,
					}
				}
				_=>row = 0,
			}
		} else{
			continue;
		}
	}
	false
}