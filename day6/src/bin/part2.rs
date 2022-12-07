use std::{collections::HashSet, fs::File, hash::Hash, io::Read};

struct UniqueStream<T: Copy + Hash + Eq> {
    lookup_table: HashSet<T>,
}

impl<T: Copy + Hash + Eq> UniqueStream<T> {
    pub fn new() -> Self {
        Self {
            lookup_table: HashSet::new(),
        }
    }

    pub fn clear(&mut self) {
        self.lookup_table.clear();
    }

    pub fn put(&mut self, t: T) {
        if !self.lookup_table.insert(t) {
            self.clear();
        }
    }

    pub fn len(&self) -> usize {
        self.lookup_table.len()
    }
}

fn main() {
    let mut unique_stream = UniqueStream::new();
    for (i, c) in File::open("input")
        .unwrap()
        .bytes()
        .map(|b| b.unwrap())
        .enumerate()
    {
        unique_stream.put(c);
        if unique_stream.len() == 14 {
            println!("Found the start of message at {}", i + 1);
            break;
        }
    }
}
