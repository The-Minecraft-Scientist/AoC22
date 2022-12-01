use std::collections::HashMap;
use std::env;
use std::str::FromStr;

#[macro_use]
extern crate load_file;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_in = load_str!("inp.txt");
    let split: Vec<&str> = file_in.split("\n").collect();
    let mut elves = Vec::new();
    println!("{:?}",split);
    let mut current = 0;
    for item in split {
        if item.clone() == "" {
            elves.push(current);
            current = 0;
        } else {
            current += i32::from_str(item).unwrap();
        }
    }
    elves.sort();
    let ilen = elves.len() - 1;
    println!("{:?}", elves[ilen]+elves[ilen-1]+elves[ilen - 2])
}
