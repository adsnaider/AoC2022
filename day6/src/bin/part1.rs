use std::{fs::File, io::Read};

use itertools::Itertools;

fn main() {
    let start_of_message = File::open("input")
        .unwrap()
        .bytes()
        .map(|b| b.unwrap())
        .tuple_windows()
        .enumerate()
        .find(|(_, (a1, a2, a3, a4))| {
            a1 != a2 && a1 != a3 && a1 != a4 && a2 != a3 && a2 != a4 && a3 != a4
        })
        .map(|(i, _)| i + 4)
        .unwrap();

    println!("Found the start of message at {start_of_message}");
}
