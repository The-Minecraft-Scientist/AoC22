#[macro_use]
extern crate load_file;

fn main1() {
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
fn main() {
    let file_in = load_str!("foo2.txt");
    let split: Vec<&str> = file_in.split("\n").collect();
    let mut score_accum = 0usize;
    let mut local_accum: Vec<&str> = vec![];
    for index in 0..(split.len()) {
        let item = *split.get(index).unwrap();
        local_accum.push(item);
        dbg!(item);
        if local_accum.len() == 3 {
            for item in local_accum[0].chars() {
                if local_accum[1].contains(item) && local_accum[2].contains(item) {
                    println!("{:?}",item);
                    score_accum += priority(&item);
                    break
                }
            }
            local_accum.clear();
        }
    };
    dbg!(score_accum);
}
fn priority(char: &char) -> usize {
    let score;
    if char.is_ascii_uppercase() {
        score = ((*char as u8) - 38) as usize;
    } else {
        score = ((*char as u8) - 96) as usize;
    };
    score
}
