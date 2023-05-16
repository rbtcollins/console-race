use std::{io::Write, thread::JoinHandle};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> std::io::Result<()> {
    let mut handles: Vec<JoinHandle<std::io::Result<()>>> = vec![];
    for color in [
        Color::Black,
        Color::Blue,
        Color::Cyan,
        Color::Green,
        Color::Magenta,
        Color::Red,
        Color::White,
        Color::Yellow,
    ] {
        let spec = ColorSpec::new().set_fg(Some(color)).clone();
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        handles.push(std::thread::spawn(move || -> std::io::Result<()> {
            loop {
                stdout.set_color(&spec)?;
                writeln!(&mut stdout, "{:?} text!", spec)?;
            }
            // unreachable!();
            // Ok(())
        }));
    }

    for handle in handles {
        handle.join().unwrap()?;
    }
    Ok(())
}
