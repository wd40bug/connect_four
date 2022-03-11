use crate::solver::WIDTH;

pub struct Sorter{
    entries: [Entry; WIDTH as  usize],
    size: u32,
}
impl Sorter{
    pub fn add(&mut self, play: u64, score: i32){
        self.size+=1;
        let mut pos =self.size as usize;
        while self.entries[pos - 1].score > score{
            self.entries[pos] = self.entries[pos - 1].clone();
            pos-=1;
        }
        self.entries[pos].play = play;
        self.entries[pos].score = score;

    }
    pub fn get_next(&mut self)->u64{
        return if self.size != 0{
            self.size-=1;
            self.entries[self.size as usize].play
        } else{
            0
        }
    }
    pub fn new()->Sorter{
        Sorter{
            entries: [Entry{score: 0, play: 0},Entry{score: 0, play: 0},Entry{score: 0, play: 0},Entry{score: 0, play: 0},Entry{score: 0, play: 0},Entry{score: 0, play: 0},Entry{score: 0, play: 0}],
            size: 0,
        }
    }
}
#[derive(Clone)]
struct Entry{
    play: u64,
    score: i32,
}