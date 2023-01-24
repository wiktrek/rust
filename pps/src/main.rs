use std::env::{args, Args};
fn main() {
let mut args: Args= args();
let pops: i32 = args.nth(1).unwrap().parse().unwrap();
let time: i32 = args.nth(0).unwrap().parse().unwrap();
let result: i32 = pops / time;
println!("{pops} / {time} = {result}")
}