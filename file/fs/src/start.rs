use std::io;
mod create;
mod options;
mod remove;
mod rename;
use create::create;
use options::options;
use remove::remove;
use rename::rename;
pub fn start() -> Result<(), std::io::Error> {
    let mut action: String = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Couldn’t read from stdin");
    let replace = action.replace("\r\n", "").replace(" ", "");
    let a: &str = replace.as_str();
    match a {
        "create" => create(),
        "remove" => remove(),
        "rename" => return rename(),
        "options" => options(),
        _ => panic!("error, picked: {}", a),
    }
}
