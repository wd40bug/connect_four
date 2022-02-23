struct Solver{
    node_count: u64
}
impl Solver{
    fn negamax(&mut self, pos: &Position)->i32{
        self.node_count+=1;
        if pos.moves == 42{
            return  0;
        }

        for x in 0usize..7{
            if pos.can_play(x) && pos.is_winning_move(x){
                return (43 - pos.moves)/2;
            }
        }

        let best_score = -42;

        for x in 0..7{
            if pos.can_play(x){
                let mut pos2 = pos;
                pos2.play(x);
                let score = -self.negamax(pos2);
                best_score = score.max(best_score);
            }
        }
    }
    pub fn solve(&mut self, pos: &Position) -> i32{
        self.node_count = 0;
        negamax(pos)
    }
    
}