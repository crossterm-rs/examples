//! Switching between input modes.

use crossterm::{Result, RawScreen, Terminal, TerminalInput, ClearType, InputEvent, KeyEvent, Crossterm};
use std::{thread, time::Duration};

fn test(terminal: &Terminal, input: &TerminalInput) -> Result<()> {
    let _raw = RawScreen::into_raw_mode()?;
    terminal.clear(ClearType::All)?;
    terminal.write("press `q` to exit test")?;

    let mut stdin = input.read_async();

    loop {
        if let Some(event) = stdin.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Char('q')) => break,
                _ => (),
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    terminal.clear(ClearType::All)?;
    Ok(())
}

// cargo run --example mixed_input
fn main() -> Result<()> {
    let crossterm = Crossterm::new();
    let terminal = crossterm.terminal();
    let input = crossterm.input();

    loop {
        println!("Type `exit` or `test`");
        let line = input.read_line()?;

        match line.as_str() {
            "test" => test(&terminal, &input)?,
            "exit" => break,
            _ => println!("try `exit` or `test`"),
        }
    }

    Ok(())
}
