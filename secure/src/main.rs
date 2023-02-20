#![allow(unused)]
use std::fs;
use std::io::{prelude::*, Error};
use std::path::Path;

fn main() {
    let path = Path::new("secure.txt");
   
    secure(read_file(path))
}
fn read_file(path: &Path) -> String {

    let f = fs::read_to_string(path);
    f.unwrap()
}
fn secure(f: String) {
let mut e: Vec<&str> = vec![];
 println!("{}", f);
 let char_vec: Vec<char> = f.chars().collect();
     for c in char_vec {
 match c {
    e => e.push("")

 }
}
println!("{:?}", e)
}