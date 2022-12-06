#[macro_use]
extern crate load_file;

fn main() {
    let file_in = load_str!("foo.txt");
    let mut accum = 0usize;
    for itm in file_in.split("\n") {
        let item: Vec<char> = itm.chars().collect();
        let (c1, c2) = item.split_at(item.len()/2);
        for char in c1 {
            if c2.contains(&char) {
                print!("found match: {:?} ", &char);
                print!("code: {:?} ", *char as u8);
                let score;
                if char.is_ascii_uppercase() {
                    score = ((*char as u8) - 38) as usize;
                } else {
                    score = ((*char as u8) - 96) as usize;
                }
                print!("scored as {:?} \n", score);
                accum += score;
                break
            }
        }
    }
    println!("{:?}",accum)
}
