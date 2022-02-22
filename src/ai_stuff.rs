use crate::game_board::GameBoard;

struct Position{
    board: [[u8; 6];7],
    height: [u8;7],
    moves: u32,
}
impl Position{
    fn can_play(&self, col: u32)->bool{
        height[col] < 6
    }
    fn play(&mut self, col: u32){
        self.board[col][self.height[col]] = 1+self.moves%2;
        self.height[col]+=1;
        self.moves+=1;
    }
    fn is_winning_move(&self, col: u32)->bool{
        let current_player = 1+self.moves%2;
        if self.height[col] <=3 
            && self.board[col][self.height[col]-1] == current_player
            && self.board[col][self.height[col]-2] == current_player
            && self.board[col][self.height[col]-3] == current_player 
            {return true;}
        for i in -1..=1{
            let mut nb = 0;
            for i in (-1..=1).step_by(2){
                for x in (col+dx..)
            }
        }
    }
}