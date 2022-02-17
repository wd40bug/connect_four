use connect_four::game_board::GameBoard;
use connect_four::piece::Piece;
fn main(){
    let mut board = GameBoard::new();
    board.place(Piece::Red,2);
    println!("{}",board);
}