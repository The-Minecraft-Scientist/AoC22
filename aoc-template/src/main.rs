use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_in = load_str!(args[0]);
    let out = "";
    println!("{}", out);
}
