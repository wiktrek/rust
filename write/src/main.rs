use std::fs;
use std::io;
use std::io::prelude::*;
fn main() {
    let stdin = io::stdin();
    let file = fs::read_to_string("./text.json").expect("err");
    println!("{}", file);
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
