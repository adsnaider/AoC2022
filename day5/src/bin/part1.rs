use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
struct Instr {
    from: usize,
    to: usize,
    count: usize,
}

impl Instr {
    pub fn apply<T>(&self, stacks: &mut [Vec<T>]) {
        for _ in 0..self.count {
            let t = stacks[self.from].pop().unwrap();
            stacks[self.to].push(t);
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instr = s.split(' ');
        instr.next();
        let count = instr.next().unwrap().parse().unwrap();
        instr.next();
        let from = instr.next().unwrap().parse::<usize>().unwrap() - 1;
        instr.next();
        let to = instr.next().unwrap().parse::<usize>().unwrap() - 1;

        Ok(Self { from, to, count })
    }
}

fn main() {
    let mut stacks = day5::starting_stacks();

    BufReader::new(File::open("input").unwrap())
        .lines()
        .skip(10)
        .map(|line| -> Instr { line.unwrap().parse().unwrap() })
        .for_each(|instr| instr.apply(&mut stacks));

    println!("The secret message is");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}
