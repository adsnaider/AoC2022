use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Item(u8);

impl Item {
    pub fn new(c: char) -> Self {
        assert!(c.is_ascii_alphabetic());
        Self(c.try_into().unwrap())
    }

    pub fn priority(&self) -> u8 {
        match self.0 {
            b'a'..=b'z' => self.0 - b'a' + 1,
            b'A'..=b'Z' => self.0 - b'A' + 27,
            x => panic!("Unexpected item type: {x}"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Pouch {
    items: Vec<Item>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rucksack {
    p1: Pouch,
    p2: Pouch,
}

impl Rucksack {
    pub fn in_both(&self) -> Item {
        let in_p1: HashSet<Item> = self.p1.items.iter().copied().collect();
        *self
            .p2
            .items
            .iter()
            .find(|item| in_p1.contains(item))
            .unwrap()
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() % 2 == 0);
        let (p1, p2) = s.split_at(s.len() / 2);
        let p1 = Pouch {
            items: p1.chars().map(Item::new).collect(),
        };
        let p2 = Pouch {
            items: p2.chars().map(Item::new).collect(),
        };
        assert!(p1.items.len() == p2.items.len());

        Ok(Self { p1, p2 })
    }
}

fn main() {
    let priority_sum: usize = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|line| Rucksack::from_str(&line.unwrap()).unwrap())
        .map(|rucksack| rucksack.in_both())
        .map(|item| item.priority() as usize)
        .sum();

    println!("Duplicate items priority: {priority_sum}");
}
