use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use std::path::Path;
use std::process::{Command, Output};
fn main() {
    let options = vec!["Pnpm", "Yarn", "npm", "Other"];
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
    let mut pnpm = Command::new("cmd");
    // let mut yarn = Command::new("yarn");
    // let mut npm = Command::new("npm");

    // pnpm.current_dir(dir);
    let a: Output = pnpm
        .args(["/C", format!("pnpm i {}", package).as_str()])
        .output()
        .expect("process failed to execute");

    print!("e {:?}", a)
}
