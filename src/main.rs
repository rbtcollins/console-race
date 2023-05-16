use std::{io::Write, thread::JoinHandle};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn unlocked_io(spec: ColorSpec) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    loop {
        stdout.set_color(&spec)?;
        writeln!(&mut stdout, "{:?} text!", spec)?;
    }
}

fn locked_io(spec: ColorSpec) -> std::io::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    loop {
        {
            let mut w = stdout.lock();
            w.set_color(&spec)?;
            writeln!(&mut w, "{:?} text!", spec)?;
        }
    }
}

fn main() -> std::io::Result<()> {
    let locked = std::env::args().count() == 2;
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
        handles.push(std::thread::spawn(move || -> std::io::Result<()> {
            if locked {
                locked_io(spec)
            } else {
                unlocked_io(spec)
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
