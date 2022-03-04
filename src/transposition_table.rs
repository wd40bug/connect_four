pub struct TranspositionTable{
    entries: Vec<Entry>,
}
impl TranspositionTable{
    fn index(&self, key: u64) -> usize{
        key as usize% self.entries.len()
    }
    pub fn put(&mut self, key: u64, val: u8){
        let i = self.index(key);
        self.entries[i].key = key;
        self.entries[i].val = val;
    }
    pub fn new(size: u64)->TranspositionTable{
        TranspositionTable{
            entries: vec![Entry{key: 0, val: 0};size as usize],
        }
    }
    pub fn get(&self, key: u64)->u8{
        let i = self.index(key);
        if self.entries[i].key==key{
            return self.entries[i].val;
        }else{
            return 0;
        }
    }
    pub fn reset(&mut self){
        for i in 0..self.entries.len() {
            self.entries[i] = Entry{val: 0, key: 0};
        }
    }

}
#[derive(Clone)]
struct Entry{
    key: u64,
    val: u8,
}