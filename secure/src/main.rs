#![allow(unused)]
use std::fs;
use std::io::{prelude::*, Error};
use std::path::Path;

fn main() {
    let path = Path::new("secure.txt");
   
    secure(read_file(path))
}
