use std::io;
use chrono::prelude::*;
fn main() {
    let local_time = Local::now().timestamp();
    let mut user_input = String::new();
    println!("enter date");
    io::stdin()
    .read_line(&mut user_input).expect("Couldn’t read from stdin");
let b = Utc.with_ymd_and_hms(2017, 04, 02, 12, 50, 32).unwrap().timestamp();
let a= local_time - b;

println!("{}, {}", a, local_time);
}
