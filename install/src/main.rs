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
    let mut cmd = Command::new("cmd");
    if selection == 0 {
        let a: Output = cmd
            .current_dir(dir)
            .args(["/C", format!("pnpm i {}", package).as_str()])
            .output()
            .expect("process failed to execute");
        println!("{:?}", a.stdout);
    }
    if selection == 1 {
        let a: Output = cmd
            .current_dir(dir)
            .args(["/C", format!("yarn add {}", package).as_str()])
            .output()
            .expect("process failed to execute");
        println!("{:?}", a.stdout);
    }
    if selection == 2 {
        let a: Output = cmd
            .current_dir(dir)
            .args(["/C", format!("npm i {}", package).as_str()])
            .output()
            .expect("process failed to execute");
        println!("{:?}", a.stdout);
    }
}
