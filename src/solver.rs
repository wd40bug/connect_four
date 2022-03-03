use crate::ai_stuff::Position;



pub struct Solver{
    pub node_count: u64,
    pub column_order: [usize;7],
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

        let max = (41-pos.moves)/2;
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
        return alpha;
    }
    pub fn solve(&mut self, pos: &Position) -> i32{
        self.node_count = 0;
        self.negamax(pos, -21, 21)
    }

}