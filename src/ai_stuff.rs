use crate::game_board::GameBoard;

pub struct Position{
    board: [[u32; 6];7],
    height: [u32;7],
    moves: u32,
}
impl Position{
    fn can_play(&self, col: usize)->bool{
        self.height[col] < 6
    }
    fn play(&mut self, col: usize){
        self.board[col][self.height[col] as usize] = 1+self.moves%2;
        self.height[col]+=1;
        self.moves+=1;
    }
    fn is_winning_move(&self, col: usize)->bool{
        let current_player = 1+self.moves%2;
        if self.height[col] <=3 
            && self.board[col][self.height[col] as usize-1] == current_player
            && self.board[col][self.height[col] as usize-2] == current_player
            && self.board[col][self.height[col] as usize-3] == current_player 
            {return true;}
        for dy in -1i32..=1{
            let mut nb = 0;
            for dx in (-1i32..=1).step_by(2){
                let mut x = col as i32+dx;
                let mut y = self.height[col]as i32+dx*dy;
                while x >= 0 && x < y && y >= 0 && y < 6 && self.board[x as usize][y as usize] == current_player{
                    x+=dx;
                    y+=dy*dx;
                    nb+=1;
                }
            }
            if nb >= 3{
                return true;
            }
        }
        false
    }
    fn new()->Position{
        Position{
            board: [[0;6];7],
            height: [0;7],
            moves: 0,
        }
    }

}