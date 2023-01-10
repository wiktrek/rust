mod get_file;
use std::fs;
fn main() {
get_file::run();
let file = fs::read_to_string("encrypt.txt").expect("Should have been able to read the file");
println!("{file}")
}
