use std::error::Error;
use csv;

fn main() {
    if let Err(e) = read_from_file("./e.csv") {
        eprintln!("{}", e)
    }
}
