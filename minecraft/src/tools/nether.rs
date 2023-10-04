use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};

pub fn nether() {
    let x: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("x")
        .interact_text()
        .unwrap();
    let y: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("y")
        .interact_text()
        .unwrap();
    let z: i32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("z")
        .interact_text()
        .unwrap();
    let options = ["Overworld -> Nether", "Nether -> Overworld"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("minecraft")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    match selection {
        0 => println!("x:{}\ny:{}\nz:{}", (x / 8), y, (z / 8)),
        1 => println!("x:{}\ny:{}\nz:{}", (x * 8), y, (z * 8)),
        _ => println!("Error"),
    }
}
