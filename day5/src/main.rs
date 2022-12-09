#[macro_use]
extern crate load_file;

use std::str::FromStr;

fn main() {
    let mut stacks = vec![];
    stacks.push(vec!["R", "N", "P", "G"]);
    stacks.push(vec!["T", "J", "B", "L", "C", "S", "V", "H"]);
    stacks.push(vec!["T", "D", "B", "M", "N", "L"]);
    stacks.push(vec!["R", "V", "P", "S", "B"]);
    stacks.push(vec!["G", "C", "Q", "S", "W", "M", "V", "H"]);
    stacks.push(vec!["W", "Q", "S", "C", "D", "B", "J"]);
    stacks.push(vec!["F", "Q", "L"]);
    stacks.push(vec!["W", "M", "H", "T", "D", "L", "F", "V"]);
    stacks.push(vec!["L", "P", "B", "V", "M", "J", "F"]);
   /* stacks.push(vec!["Z", "N"]);
    stacks.push(vec!["M", "C", "D"]);
    stacks.push(vec!["P"]);*/
    let file_in = load_str!("foo.txt");
    let split: Vec<Vec<&str>> = file_in.split("\n").map(|x| -> Vec<&str> {
        x.split(" ").collect::<Vec<&str>>()
    }).collect();
    for line in split {
        println!("{:?}", &line);
        dbg!(&stacks);
        let from = u32::from_str(line[3]).unwrap() as usize -1;
        let final_index = (&stacks[from].len() - 1) - (u32::from_str(line[1]).unwrap() as usize - 1);
        let to = u32::from_str(line[5]).unwrap() as usize -1;
        dbg!(&from);
        dbg!(&final_index);
        dbg!(&to);

        let mut split_off = stacks[from].split_off(final_index);
        //part one
        // split_off.reverse();
        stacks[to].append(&mut split_off);
    }
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap())
    }
}

