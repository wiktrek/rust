use dialoguer::{theme::ColorfulTheme, FuzzySelect};
mod tools;
use tools::*;
fn main() {
    let options = ["Hopper clock", "Coords"];
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
