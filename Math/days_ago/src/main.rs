use std::io;
use chrono::prelude::*;
fn main() {
    let local_time = Local::now().timestamp();
    let mut day = String::new();
    println!("enter the day");
    io::stdin()
    .read_line(&mut day).expect("Couldn’t read from stdin");
let mut month = String::new();
println!("enter the month");
io::stdin()
.read_line(&mut month).expect("Couldn’t read from stdin");
let mut year = String::new();
println!("enter the year");
io::stdin()
.read_line(&mut year).expect("Couldn’t read from stdin");
let b = Utc.with_ymd_and_hms(year.parse().unwrap(), month.parse().unwrap(), day.parse().unwrap(), 12, 50, 0).unwrap().timestamp();
let a= local_time - b;
let c = a / 60 / 60 / 24;
println!("{}, {}", c, local_time);
}
