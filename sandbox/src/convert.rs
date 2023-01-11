pub fn convert(s: &str,) -> i32{
    println!("convert.rs:");
    let mut value = 0;
let  mut a = 0;
for chars in s.chars().rev() {
let b = get_value(chars);
    if b >= a {
    value += b;

} else {
    value -= b;
}
a = b
}

    println!("{}", value);
    return value;
}
fn get_value(cha: char) -> i32 {
    match cha {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}