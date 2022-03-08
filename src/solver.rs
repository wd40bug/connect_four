

use crate::{ai_stuff::Position, transposition_table::TranspositionTable};

pub const HEIGHT: i32 = 6;
pub const WIDTH: i32 = 7;
pub const MIN_SCORE: i32 = -((HEIGHT * WIDTH) as i32) / 2 + 3;
pub const MAX_SCORE: i32 = ((WIDTH * HEIGHT) as i32 + 1) / 2 - 3;

pub struct Solver{
    pub node_count: u64,
    pub column_order: [usize;7],
    pub transposition_table: TranspositionTable,
}


impl Solver{
    fn negamax(&mut self, pos: &Position, alpha: i32, beta: i32)->i32{
        // log::info!("negamaxing: {}",ansi_term::Color::Purple.paint(&pos.seq));
        let mut alpha = alpha;
        let mut beta = beta;
        self.node_count+=1;
        let next = pos.possible_non_loosing_moves();
        if next==0 {
            return -(WIDTH*HEIGHT-pos.moves as i32)/2;
        }
        if pos.moves as i32>=WIDTH*HEIGHT-2{
            return 0;
        }
        let min = -(WIDTH*HEIGHT-2 - pos.moves as i32)/2;
        if alpha < min{
            alpha = min;
            if alpha>=beta{return alpha;}
        }
        let mut max = (HEIGHT*WIDTH-1 - pos.moves as i32)/2;
        let val = self.transposition_table.get(pos.key());
        if val != 0{
            max = val as i32 + MIN_SCORE -1;
        }
        if beta > max{
            beta = max;
            if alpha>=beta{return beta;}
        }
        for x in 0..WIDTH as usize{
            if next & Position::column_mask(self.column_order[x] as u64)!=0{
                let mut pos2 = pos.clone();
                pos2.play(self.column_order[x]);
                let score = -self.negamax(&pos2, -alpha, -beta);
                if score>=beta{return score;}
                if score>alpha{alpha = score}
            }
        }
        self.transposition_table.put(pos.key(), (alpha - MIN_SCORE +1) as u8);
        alpha
    }



    pub fn solve(&mut self, pos: &Position) -> i32{
        if pos.can_win_next(){
            return (WIDTH*HEIGHT+1-pos.moves as i32)/2;
        }
        let mut min = -(HEIGHT*WIDTH - pos.moves as i32)/2;
        let mut max:i32 = (HEIGHT*WIDTH+1 - pos.moves as i32)/2;
        while min<max {
            let mut med = min + (max - min)/2;
            med = if med <= 0 && min/2<med{
                min/2
            } else if med >=0 && max/2 > med{
                max/2
            } else{
                med
            };
            let r =self.negamax(pos, med, med + 1);
            if r<=med {
                max = r;
            } else{
                min = r;
            }
        }
        min
    }

}