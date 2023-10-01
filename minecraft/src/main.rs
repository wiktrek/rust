use dialoguer::{
    theme::ColorfulTheme,
    // FuzzySelect
    Input,
};
fn main() {
    // let options = vec!["Programming", "Reading", "Learning rust", "Other"];
    // let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
    //     .with_prompt("what is your hobby")
    //     .default(0)
    //     .items(&options[..])
    //     .interact()
    //     .unwrap();
    println!("hopper clock");
    let blocks: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blocks")
        .interact_text()
        .unwrap();
    println!("seconds: {}", blocks * 0.4);
}
