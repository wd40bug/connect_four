pub fn seq(){
    if let Some(_) = std::env::args().nth(3){
        let mut seq = if let Some(seq) = std::env::args().nth(2) {
            seq
        } else {
            let mut foo = String::new();
            std::io::stdin().read_line(&mut foo).unwrap();
            foo.trim().to_string()
        };
        simple_logging::log_to_stderr(LevelFilter::Debug);
        let mut solver = Solver {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            transposition_table: TranspositionTable::new(8388593),
        };
        let mut pos = Position::new();
        if !pos.set_up(seq.clone()) {
            log::warn!("You have messed up");
        } else {
            let current_player = pos.moves%2;
            loop {
                seq = String::new();
                let now = pos.moves%2;
                let score = if now == current_player{
                    solver.solve(&pos, &None)
                } else{
                    -solver.solve(&pos, &None)
                };
                log::debug!("Position: {}, Score: {}", ansi_term::Color::Cyan.paint(&pos.seq).to_string(), score);
                std::io::stdin().read_line(&mut seq).unwrap();
                seq = seq.trim().to_string();
                match &seq[..]{
                    ".."=>{
                        let mut foo = pos.seq.clone();
                        foo.pop();
                        pos = Position::new();
                        pos.set_up(foo);
                    },
                    "ls"=>{
                        for x in 0..7{
                            if pos.possible_non_loosing_moves() & Position::column_mask(x)!=0{
                                let mut pos2 = pos.clone();
                                pos2.play_col(x);
                                log::debug!("Score: {}, Play: {}",-solver.solve(&pos2, &None), x);
                            }
                        }
                    }
                    _=>{
                        if let Ok(col) = seq.parse(){
                            pos.play(col);
                        }
                    },
                }
            }
        }
    } else{
        let seq = if let Some(seq) = std::env::args().nth(2) {
            seq
        } else {
            let mut foo = String::new();
            std::io::stdin().read_line(&mut foo).unwrap();
            foo.trim().to_string()
        };
        simple_logging::log_to_stderr(LevelFilter::Debug);
        let mut solver = Solver {
            node_count: 0,
            column_order: [3, 2, 4, 1, 5, 0, 6],
            transposition_table: TranspositionTable::new(8388593),
        };
        let mut pos = Position::new();
        if !pos.set_up(seq.clone()) {
            log::warn!("You have messed up");
        } else {
            let now = SystemTime::now();
            print_seq(&seq);
            log::info!("position: {}",pos.seq);
            let score = solver.solve(&pos, &None);
            log::info!("seq: {}, pos: {}, score: {}, time: {}\u{00B5}, nodes: {}, time per node: {}",&seq,pos.current_position,score,now.elapsed().unwrap().as_millis(),solver.node_count, solver.node_count as u128/now.elapsed().unwrap().as_micros());
        }
    }
}