use crate::{game_board::GameBoard, piece::Piece};

pub struct Directions{
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
	pub fn new()->Directions{
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
	pub fn check_directions(&mut self,x:usize, y:usize, distance:usize, board: &GameBoard, color: &Piece ){
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
		if self.north{
			self.north = check_spot(board,x,y+distance,color);
		}
		if self.south{
			self.south = check_spot(board,x,y-distance,color);
		}
		if self.west{
			self.west = check_spot(board,x-distance,y,color);
		}
		if self.east{
			self.east = check_spot(board,x+distance,y,color);
		}
		if self.north_west{
			self.north_west = check_spot(board,x-distance,y+distance,color);
		}
		if self.north_east{
			self.north_east = check_spot(board,x+distance,y+distance,color);
		}
		if self.south_west{
			self.south_west = check_spot(board,x-distance,y-distance,color);
		}
		if self.south_east{
			self.south_east = check_spot(board,x+distance,y-distance,color);
		}
	}
	pub fn check_if_any_true(&self)->bool{
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