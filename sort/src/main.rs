use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut err: bool;
    err = true;
    if err {
        error("an error has occurred").unwrap();
        err = false;
    }
    println!("{}", err)
}
fn error(error: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(&mut stdout, "{}", error).unwrap();

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    Ok(())
}
