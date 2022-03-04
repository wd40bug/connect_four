use crate::{ai_stuff::Position, transposition_table::TranspositionTable};


/// The minimum possible score of a position
pub const MIN_SCORE: i32 = -((6 * 7) as i32) / 2 + 3;
/// The maximum possible score of a postion
pub const MAX_SCORE: i32 = ((7 * 6) as i32 + 1) / 2 - 3;
pub struct Solver{
    pub node_count: u64,
    pub column_order: [usize;7],
    pub transposition_table: TranspositionTable,
}
impl Solver{
    fn negamax(&mut self, pos: &Position, alpha: i32, beta: i32)->i32{
        // log::info!("negamaxing: {}",ansi_term::Color::Purple.paint(&pos.seq));

        let mut beta = beta;
        let mut alpha = alpha;
        self.node_count+=1;
        if pos.moves == 42{
            return  0;
        }

        for x in 0usize..7{
            if pos.can_play(self.column_order[x]) && pos.is_winning_move(self.column_order[x]){
                return (43 - pos.moves) as i32/2;
            }
        }
        let mut max = (6*7-1 - pos.moves) as i32/2;

        let val = self.transposition_table.get(pos.key()) as i32;
        if val != 0{
            max = val + MIN_SCORE - 1;
        }

        if beta>max as i32{
            beta = max as i32;
            if alpha>=beta{return beta;}
        }

        for x in 0..7{
            if pos.can_play(self.column_order[x]){
                let mut pos2 = pos.clone();
                pos2.play(self.column_order[x]);
                let score = -self.negamax(&pos2, -beta, -alpha);
                if score>=beta{return beta;}
                if score>alpha{alpha=score;}
            }
        }
        self.transposition_table.put(pos.key(), (alpha - MIN_SCORE +1) as u8);
        return alpha;
    }
    pub fn solve(&mut self, pos: &Position) -> i32{
        self.node_count = 0;
        self.negamax(pos, -21, 21)
    }

}