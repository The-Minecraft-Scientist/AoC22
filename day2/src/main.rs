#[macro_use]
extern crate load_file;

fn main() {
    let file_in = load_str!("foo.txt");
    println!("{}", file_in);
}
