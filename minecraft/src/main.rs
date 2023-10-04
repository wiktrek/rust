use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
fn main() {
    let options = ["Hopper clock", "Nether"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("minecraft")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    match selection {
        0 => hopper_clock(),
        1 => nether(),
        _ => println!("No option"),
    }
}
fn nether() {
    let x: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("x")
        .interact_text()
        .unwrap();
    let y: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("y")
        .interact_text()
        .unwrap();
    let z: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("z")
        .interact_text()
        .unwrap();
}
fn hopper_clock() {
    let blocks: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blocks")
        .interact_text()
        .unwrap();
    println!("seconds: {}", blocks * 0.4);
}
