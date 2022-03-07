use std::char;

const HEIGHT: i32 = 6;
const WIDTH: i32 = 7;
#[derive(Clone)]
pub struct Position {
    pub mask: u64,
    pub current_position: u64,
    pub moves: u32,
    pub seq: String,
}
impl Position {
    //consts
    pub const BOTTOM_MASK: u64 = Self::bottom(WIDTH, HEIGHT);
    pub const BOARD_MASK: u64 = Self::BOTTOM_MASK * ((1 << HEIGHT) - 1);
    pub const fn bottom(width: i32, height: i32) -> u64 {
        return if width == 0 {
            0
        } else {
            Self::bottom(width - 1, height) | 1 << (width - 1) * (height + 1)
        };
    }
    //static functions
    fn top_mask(col: u64) -> u64 {
        1 << ((HEIGHT - 1) + col as i32 * (HEIGHT + 1))
    }
    //gets a board with a one on the bottom of the column
    fn bottom_mask_col(col: u64) -> u64 {
        1 << col * (HEIGHT + 1) as u64
    }
    //gets a board with a 1 on all cells in column
    pub fn column_mask(col: u64) -> u64 {
        ((1 << HEIGHT) - 1) << col as i32 * (HEIGHT + 1)
    }
    //constructor
    pub fn new() -> Position {
        Position {
            current_position: 0,
            mask: 0,
            moves: 0,
            seq: String::new(),
        }
    }
    //non-static functions
    //see if column has empty space
    pub fn can_play(&self, col: usize) -> bool {
        self.mask & Self::top_mask(col as u64) == 0
    }
    //produces a key with ones on top of columns
    pub fn key(&self) -> u64 {
        self.current_position + self.mask + Self::BOTTOM_MASK
    }
    //mutator function
    //plays a given column
    pub fn play(&mut self, col: usize) {
        self.seq.push(char::from_digit(col as u32 +1, 10).unwrap());
        self.current_position ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask_col(col as u64);
        self.moves += 1;
    }
    //sets up the board from a sequence of moves
    pub fn set_up(&mut self, seq: String) -> bool {
        for (j, i) in seq.chars().enumerate() {
            let col: isize = i.to_digit(10).unwrap() as isize - 1;
            if col < 0
                || col >= WIDTH as isize
                || !self.can_play(col as usize)
                || self.is_winning_move(col as usize)
            {
                log::error!("set up failed at character{} at index {}", i, j);
                return false;
            }
            self.play(col as usize);
        }
        return true;
    }





    //current working on
    //static
    pub fn compute_winning_position(position: u64, mask: u64) -> u64{
        //vertical alignments
        let mut r = (position<<1)&(position<<2)&(position<<3);

        //horizontal
        let mut p = (position << (HEIGHT+1)) & (position << 2*(HEIGHT+1));
        r |= p & (position << 3*(HEIGHT+1));
        r |= p & (position >> (HEIGHT+1));
        p = (position >> (HEIGHT+1)) & (position >> 2*(HEIGHT+1));
        r |= p & (position << (HEIGHT+1));
        r |= p & (position >> 3*(HEIGHT+1));

        //diagonal one
        p = (position << HEIGHT) & (position << 2*HEIGHT);
        r |= p & (position << 3*HEIGHT);
        r |= p & (position >> HEIGHT);
        p = (position >> HEIGHT) & (position >> 2*HEIGHT);
        r |= p & (position << HEIGHT);
        r |= p & (position >> 3*HEIGHT);

        //diagonal 2
        p = (position << (HEIGHT+2)) & (position << 2*(HEIGHT+2));
        r |= p & (position << 3*(HEIGHT+2));
        r |= p & (position >> (HEIGHT+2));
        p = (position >> (HEIGHT+2)) & (position >> 2*(HEIGHT+2));
        r |= p & (position << (HEIGHT+2));
        r |= p & (position >> 3*(HEIGHT+2));

        r & (Self::BOARD_MASK^mask)
    }
    //non-static
    pub fn winning_position(&self) -> u64{
        Self::compute_winning_position(self.current_position, self.mask)
    }
    pub fn opponent_winning_position(&self) -> u64{
        Self::compute_winning_position(self.current_position^self.mask, self.mask)
    }
    pub fn possible(&self) -> u64{
        (self.mask + Self::BOTTOM_MASK) & Self::BOARD_MASK
    }
    pub fn can_win_next(&self) -> bool{
        (self.winning_position() & self.possible())!=0
    }
    pub fn forced_moves(&self)->u64{
        self.possible() & self.opponent_winning_position()
    }
    pub fn possible_non_loosing_moves(&self)->u64{
        let mut possible_mask = self.possible();
        let opponent_win = self.opponent_winning_position();
        let forced_moves = self.forced_moves();
        if forced_moves!=0{
            if forced_moves & (forced_moves-1) != 0{
                return 0;
            }else{
                possible_mask = forced_moves;
            }
        }

        possible_mask & !(opponent_win>>1)
    }

    //updates
    //see if the move can win the game
    pub fn is_winning_move(&self, col: usize) -> bool {
        self.winning_position() & self.possible() & Self::column_mask(col as u64)!=0
    }
}
