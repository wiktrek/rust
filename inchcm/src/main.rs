use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::io::{stdin, stdout, Write};
fn main() {
    let options = ["Inches to centimeters", "Centimeters to inches"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select option")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    match selection {
        0 => inches_to_centimeters(),
        1 => centimeters_to_inches(),
        _ => (),
    }
}
fn centimeters_to_inches() {
    let mut s = String::new();
    print!("cm: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    let inches = s.replace(['\r', '\n'], "").parse::<f32>().unwrap() / 2.54;
    let feet = (inches / 12.0) as i32;
    println!(
        "{inches}inch \n{feet} feet and {}inches",
        inches as i32 - feet * 12
    );
}
fn inches_to_centimeters() {
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
