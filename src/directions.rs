use crate::{game_board::GameBoard, piece::Piece};
#[derive(Debug)]
pub struct Directions{
	north: bool,
	north_west: bool,
	west: bool,
	south_west: bool,
	south: bool,
	south_east: bool,
	east: bool,
	north_east: bool,
	north_south_count: u32,
	west_east_count:u32,
	south_west_north_east_count:u32,
	north_west_south_east_count:u32,
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
			north_south_count: 0,
			west_east_count: 0,
			south_west_north_east_count: 0,
			north_west_south_east_count: 0,
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
			self.north = match check_spot(board,x,y+distance,color){
				true=>{
					self.north_south_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.south{
			self.south = match check_spot(board,x,y-distance,color){
				true=>{
					self.north_south_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.west{
			self.west = match check_spot(board,x-distance,y,color){
				true=>{
					self.west_east_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.east{
			self.east = match check_spot(board,x+distance,y,color){
				true=>{
					self.west_east_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.south_west{
			self.south_west = match check_spot(board,x-distance,y-distance,color){
				true=>{
					self.south_west_north_east_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.north_east{
			self.north_east = match check_spot(board,x+distance,y+distance,color){
				true=>{
					self.south_west_north_east_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.north_west{
			self.north_west = match check_spot(board,x-distance,y+distance,color){
				true=>{
					self.north_west_south_east_count+=1;
					true
				},
				false=>false,
			};
		}
		if self.south_east{
			self.south_east = match check_spot(board,x,y+distance,color){
				true=>{
					self.north_west_south_east_count+=1;
					true
				},
				false=>false,
			};
		}
	}
	pub fn check_if_any_true(&self)->bool{
		self.north||self.south||self.west||self.east||self.north_west||self.north_east||self.south_west||self.south_east
	}
	pub fn check_if_any_4(&self)->bool{
		self.north_south_count>=4||self.west_east_count>=4||self.south_west_north_east_count>=4||self.north_west_south_east_count>=4
	}
	pub fn print(&self){
		println!("<-------------------------------------------------------------------------------------------->");
		println!("Northwest: {} North:  {} Northeast:{}",self.north_west, self.north, self.north_east);
		println!("West:      {}               East      {}",self.west,self.east);
		println!("Southwest: {} South:  {} Southeast:{}",self.south_west,self.south,self.south_east);
		println!("+____________________________________________________________________________________________+");
		println!("North to south; {}, West to east: {}, Southwest to Northeast: {}, Northwest to Southeast:{}"
			,self.north_south_count, self.west_east_count, self.south_west_north_east_count, 
			self.north_west_south_east_count);
		println!("<-------------------------------------------------------------------------------------------->");
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
