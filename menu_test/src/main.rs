use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
fn main() {
    let options = vec!["Programming", "Other"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("what is your hobby")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your name")
        .interact_text()
        .unwrap();
    println!(
        "your name is: {}, and your hobby is {}",
        name, options[selection]
    )
}
