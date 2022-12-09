#[macro_use]
extern crate load_file;

fn main() {
    let s = load_str!("foo.txt").chars().collect::<Vec<char>>();
    let mut i = 0usize;
    loop {
        if i + 14 >= (s.len() - 1) {break}
        let c = &s[i..i+14];
        if all_unique(c.to_vec()) {
            break;
        }
        i += 1;
    };
    println!("{}", i + 14);
}
fn all_unique(rhs: Vec<char>) -> bool {
    let mut out = true;
    for item in rhs.iter() {
        if rhs.iter().filter(|&x| x == item).count() != 1 {
            out = false;
        }
    }
    out
}
