use rand::Rng;
use std::fs;
use std::io;
use std::io::prelude::*;
fn main() {
    let stdin = io::stdin();
    let file = fs::read_to_string("./text.json")
        .expect("err")
        .replace(['[', ']'], "")
        .replace('"', "");
    let num = rand::thread_rng().gen_range(0..100);
    println!("{}", num);
    println!("{}", file);
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
