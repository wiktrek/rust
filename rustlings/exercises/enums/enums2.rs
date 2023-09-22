// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.
//
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}
#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit, // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
    fn Echo(&self, msg: String) {
        println!("{:?} {}", self, msg);
    }
    fn ChangeColor(&self, red: i32, green: i32, blue: i32) {
        println!("{:?} {} {}", red, green, blue);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
