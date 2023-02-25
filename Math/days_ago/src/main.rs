use std::io;
use chrono::{DateTime, FixedOffset, Local, Utc};
fn main() {
    let local_time = Local::now().timestamp();
    let mut user_input = String::new();
    println!("enter date");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");
let b = DateTime::timestamp();
let a= local_time - b.

println!("{}, {}", a, local_time);
}
