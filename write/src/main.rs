use rand::Rng;
use std::fs;
use std::io;
use std::io::prelude::*;
fn main() {
    let stdin = io::stdin();
    let file = trim_whitespaces(
        fs::read_to_string("./text.json")
            .expect("err")
            .replace(['[', ']', '"', '\n', '\r'], ""),
    );
    let text: Vec<&str> = file.split(", ").collect();

    println!("{:?}", text);

    let num = rand::thread_rng().gen_range(0..text.len());
    println!("{} ", text[num]);
    for line in stdin.lock().lines() {
        if line.unwrap() == text[num] {
            println!("✅")
        } else {
            println!("❌")
        }
    }
}
fn trim_whitespaces(s: String) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}
