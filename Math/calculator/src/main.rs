use std::env::{args, Args};
fn main() {
let mut args: Args= args();
let first: String = args.nth(1).unwrap();
let operator: char = args.nth(0).unwrap().chars().next().unwrap();
let second: String = args.nth(0).unwrap();
let first_number: f32 = first.parse().unwrap();
let second_number: f32 = second.parse().unwrap();
let result: f32 = operate(operator, first_number, second_number);
println!("{:?}", output(first_number, second_number, operator, result))

}
fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '*' => first_number * second_number,
    '/' => first_number / second_number,
    _ => panic!("Invalid operator used.")
}
}

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
