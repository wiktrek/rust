use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <number> <number>");
        println!("Example: 1 2 output: 50%");
        return;
    }
    let a: f64 = args[1].parse::<f64>().unwrap();
    let b: f64 = args[2].parse::<f64>().unwrap();
    let percentage: f64 = a / b * 100 as f64;
    println!("{:?}%", percentage);
}



