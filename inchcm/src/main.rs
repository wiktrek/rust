// 5′0″ 5′6″
fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("feet: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    let mut s2 = String::new();
    print!("inches: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s2)
        .expect("Did not enter a correct string");
    let feet = s.replace(['\r', '\n'], "");
    println!("{feet}  {s2}");
    let inches: f32 =
        feet.parse::<f32>().unwrap() * 12.0 + s2.replace(['\r', '\n'], "").parse::<f32>().unwrap();
    println!("{}cm", inches * 2.54);
}
