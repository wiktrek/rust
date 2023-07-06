use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use std::fs;
use std::path::Path;
fn main() {
    let options = vec!["Programming", "Reading", "Learning rust", "Other"];
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
    );
    let write = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to save those informations?")
        .default(1)
        .items(&[true, false])
        .interact()
        .unwrap();
    if write == 0 {
        let input: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter path")
            .interact_text()
            .unwrap();
        let path = Path::new(&input);
        fs::write(
            path,
            format!(
                "{{ \n \"name \": \"{}\", \n \"hobby\": \"{}\" \n}}",
                name, options[selection]
            ),
        )
        .unwrap()
    }
}
