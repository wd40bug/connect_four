use crate::ai_stuff::Position;
use ansi_term::Color::Purple;

pub struct Solver{
    pub node_count: u64
}
impl Solver{
    fn negamax(&mut self, pos: &Position)->i32{
        log::info!("negamaxing {}", Purple.paint(&pos.seq));
        self.node_count+=1;
        if pos.moves == 42{
            return  0;
        }

        for x in 0usize..7{
            if pos.can_play(x) && pos.is_winning_move(x){
                return (43 - pos.moves) as i32/2;
            }
        }

        let mut best_score = -42;

        for x in 0..7{
            if pos.can_play(x){
                let mut pos2 = pos.clone();
                pos2.play(x);
                let score = -self.negamax(&pos2);
                best_score = if score > best_score{score} else {best_score};
            }
        }
        return best_score;
    }
    pub fn solve(&mut self, pos: &Position) -> i32{
        self.node_count = 0;
        self.negamax(pos)
    }

}