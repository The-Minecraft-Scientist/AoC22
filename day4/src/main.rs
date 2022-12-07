#[macro_use]
extern crate load_file;

use std::str::FromStr;

pub struct Range {
    begin: u32,
    end: u32,
}
impl Range {
    pub fn from_str(rhs: &str) -> Range {
        let s: Vec<&str> = rhs.split("-").collect();
        let begin = u32::from_str(s[0]).unwrap();
        let end = u32::from_str(s[1]).unwrap();
        Range {
            begin,
            end
        }
    }
    pub fn contains(&self, rhs: &Self) -> bool {
        self.begin <= rhs.begin && self.end >= rhs.end
    }
    pub fn overlaps(&self, rhs: &Self) -> bool {
        self.begin >= rhs.begin && self.begin <= rhs.end
    }
}
fn main() {
    let mut ct = 0usize;
    let file_in = load_str!("foo.txt");
    let split: Vec<Vec<Range>> = file_in.split("\n")
        .map(|x| -> Vec<Range> {
        x.split(",")
            .map(|s| -> Range {Range::from_str(s)})
            .collect::<Vec<Range>>() }
        ).collect();
    for pair in split {
        if pair[0].overlaps(&pair[1]) || pair[1].overlaps(&pair[0]) {
            ct += 1;
        }
    }
    dbg!(ct);
}
