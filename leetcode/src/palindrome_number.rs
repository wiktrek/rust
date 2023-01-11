pub fn is_palindrome(x: i32) -> bool {
    println!("  palindrome_number.rs:");
    let is = x.to_string()==x.to_string().chars().rev().collect::<String>();
    println!("{is}");
    is
}