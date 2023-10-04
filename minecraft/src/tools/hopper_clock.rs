use dialoguer::{theme::ColorfulTheme, Input};
pub fn hopper_clock() {
    let blocks: f32 = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Blocks")
        .interact_text()
        .unwrap();
    println!("seconds: {}", blocks * 0.4);
}
