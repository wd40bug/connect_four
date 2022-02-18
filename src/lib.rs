use crate::piece::Piece;
use crate::game_board::GameBoard;
use std::io;

pub mod game_board;
pub mod piece;
pub fn run(){
	let mut board = GameBoard::new();
	loop{
		if player_turn(&mut board, Piece::Red){
			break;
		}

		if player_turn(&mut board, Piece::Blue){
			break;
		}

	}
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
 struct Directions{
	north: bool,
	north_west: bool,
	west: bool,
	south_west: bool,
	south: bool,
	south_east: bool,
	east: bool,
	north_east: bool,
}
impl Directions{
	fn new()->Directions{
		Directions{
			north: true,
			north_west: true,
			west: true,
			south_west: true,
			south: true,
			south_east: true,
			east: true,
			north_east: true,
		}
	}
	fn check_directions(&mut self,x:usize, y:usize, distance:usize, board: &GameBoard, color: &Piece ){
		if distance>y{
			self.south=false;
			self.south_east = false;
			self.south_west = false;
		}
		if distance>x{
			self.west = false;
			self.north_west = false;
			self.south_west = false;
		}
		if self.north == true {
			self.north = check_spot(board,x,y+distance,color);
		}
		if self.south == true {
			self.south = check_spot(board,x,y-distance,color);
		}
		if self.west == true {
			self.west = check_spot(board,x-distance,y,color);
		}
		if self.east == true {
			self.east = check_spot(board,x+distance,y,color);
		}
		if self.north_west == true {
			self.north_west = check_spot(board,x-distance,y+distance,color);
		}
		if self.north_east == true {
			self.north_east = check_spot(board,x+distance,y+distance,color);
		}
		if self.south_west == true {
			self.south_west = check_spot(board,x-distance,y-distance,color);
		}
		if self.south_east == true {
			self.south_east = check_spot(board,x+distance,y-distance,color);
		}
	}
	fn check_if_any_true(&self)->bool{
		self.north||self.south||self.west||self.east||self.north_west||self.north_east||self.south_west||self.south_east
	}
}
fn check_spot( board: &GameBoard, x: usize, y: usize, color: &Piece)-> bool{
	if let Ok(piece) = board.get(x,y){
		if piece.variant_eq(color){
			return true;
		}
	}
	return false;
}
fn player_turn(board: &mut GameBoard, piece: Piece)->bool{
	let mut play = String::new();
	io::stdin()
		.read_line(&mut play)
		.unwrap();
	let x: u32 = (play.trim().parse::<u32>().unwrap())-1;
	let y = board.place(piece,x.try_into().unwrap());
	println!("{}",board);
	if has_connect_4(& board,x.try_into().unwrap(),y.try_into().unwrap(),piece){
		return true;
	}
	false
}
