#[macro_use]
extern crate load_file;

use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let f = load_str!("foo.txt").split("$").collect::<Vec<&str>>()[1..].to_vec().into_iter().map(|x| x.split_at(1).1).collect::<Vec<&str>>();
    let mut current_dir: Path = Path::new();
    let mut files: HashMap<Path, usize> = HashMap::default();
    let mut folders: HashMap<Path, usize> = HashMap::default();
    println!("{:?}",f);
    for cmd in f {
        let split = cmd.split("\n").collect::<Vec<&str>>();
        match split[0].split_at(2).clone() {
            ("cd", s) => {
                match s {
                    " /" => current_dir.root(),
                    " .." => current_dir.parent(),
                    a => current_dir.sub(vec![a.to_owned()]),
                }
            }
            ("ls", _) => {
                for item in split[1..].to_owned() {
                    match item.split(" ").collect::<Vec<&str>>()[0] {
                        "dir" => {},
                        "" => {},
                        sz  => {
                            let s2 = item.split(" ").collect::<Vec<&str>>();
                            let name = s2[1];
                            let size = usize::from_str(sz).unwrap();
                            files.insert(current_dir.file(name), size);
                        }
                    }
                }
            }
            _ => {panic!("unexpected command")}
        }
    }
    dbg!(&files);
    for item in files.iter() {
        let (_file, size) = item;
        let mut file = _file.clone();
        file.parent();
        loop {
            let new_val = match folders.get(&file) {
                Some(v) => v + *size,
                None => *size,
            };
            folders.insert(file.clone(), new_val);
            file.parent();
            if file.dirs.len() == 0 {break}
        }
    }
    dbg!(&folders);
    let mut ct = 0usize;
    let mut possibles = vec![];
    for item in folders.values() {
        if &24650017 + item >= 30000000 {
            possibles.push(*item)
        }
    }
    possibles.sort();
    dbg!(possibles[0]);
}
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Path {
    dirs: Vec<String>
}

impl Path {
    fn new() -> Path {
        let mut p = Path {
            dirs: vec![]
        };
        p.root();
        p
    }
    fn sub(&mut self, sub_path: Vec<String>) {
        self.dirs.append(&mut sub_path.clone())
    }
    fn file(&self, file: &str) -> Path {
        let mut new = self.clone();
        new.sub(vec![file.to_owned()]);
        new
    }
    fn parent(&mut self) {
        let _ = self.dirs.pop();
    }
    fn root(&mut self) {
        self.dirs.clear();
        self.dirs.push("/".to_owned());
    }
}