#[macro_use]
extern crate load_file;

use std::collections::HashMap;
use crate::Weapon::{Paper, Rock, Scissors};
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Weapon {
    Rock(),
    Paper(),
    Scissors(),
}

impl Weapon {
    fn get_score(&self) -> u32 {
        match self {
            Rock() => {1}
            Paper() => {2}
            Scissors() => {3}
        }
    }
    fn get_weakness(&self) -> Weapon {
        match self {
            Rock() => {Paper()}
            Paper() => {Scissors()}
            Scissors() => {Rock()}
        }
    }
    fn get_win(&self) -> Weapon {
        match self {
            Paper() => {Rock()}
            Scissors() => {Paper()}
            Rock() => {Scissors()}
        }
    }

}
fn main() {
    let mut map: HashMap<char, Weapon> = HashMap::default();
    map.insert('A', Rock());
    map.insert('B', Paper());
    map.insert('C', Scissors());
    let file_in = load_str!("foo2.txt");
    let mut score = 0u32;
    for item in file_in.split("\n") {
        let they_played = map.get(&item.chars().nth(0).unwrap()).unwrap();
        let needed_ending = &item.chars().nth(2).unwrap();
        let you_played;
        match needed_ending {
            'X' => {you_played = they_played.get_win()},
            'Y' => {you_played = they_played.clone()},
            'Z' => {you_played = they_played.get_weakness()},
            _ => {panic!("this shouldnt happen")}
        }
        dbg!((you_played,they_played));
        score += you_played.get_score();
        if you_played == they_played.clone() {
            score += 3;
            println!("tie")
        } else if they_played.get_weakness() == you_played.clone() {
            score += 6;
            println!("you won")
        } else {
            println!("you lost")
        }
    }
    println!("{:?}", score);
}


