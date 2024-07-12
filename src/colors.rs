use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub(crate) fn write_blue() -> () {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))) {
        Ok(()) => {}
        Err(err) => eprintln!("Failed to set color: {}", err),
    }
}

pub(crate) fn write_none() -> () {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))) {
        Ok(()) => {}
        Err(err) => eprintln!("Failed to set color: {}", err),
    }
}
