mod convert;
mod valid_parentheses;
mod two_sum;
mod palindrome_number;
use palindrome_number::is_palindrome;
use convert::convert;
use two_sum::sum;

fn main() {
sum(vec![1, 2, 3,5], 7);
println!("  main.rs:");
println!("Hello World");
convert("VI");
is_palindrome(2442);
valid_parentheses::is_valid("()()()(){}".to_string());
}
