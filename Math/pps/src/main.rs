use std::env::{args, Args};
fn main() {
let mut args: Args= args();
let pops: f64 = args.nth(1).unwrap().parse().unwrap();
let time: f64 = args.nth(0).unwrap().parse().unwrap();
let result: f64 = pops / time;
println!("{pops} pops / {time}s = {result} pops per second")
}