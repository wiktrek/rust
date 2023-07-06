use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use std::path::Path;
use std::process::{Command, Output};
fn main() {
    let options = vec!["pnpm", "yarn", "npm", "Other"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a package manager")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    if selection == 3 {
        return println!("We don't support your package manager.");
    }
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter dir")
        .interact_text()
        .unwrap();
    let package: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("package:")
        .interact_text()
        .unwrap();
    let dir = Path::new(&input);
    let mut cmd = Command::new("cmd");
    let a: Output = cmd
        .current_dir(dir)
        .args([
            "/C",
            format!("{} add {}", options[selection], package).as_str(),
        ])
        .output()
        .expect("process failed to execute");
    println!("{}", std::str::from_utf8(&a.stdout).unwrap());
}
