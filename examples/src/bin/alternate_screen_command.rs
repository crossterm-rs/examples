//! This is the same than alternate_screen but using
//! the command API instead of the "old style" direct
//! unbuffered API.

use std::{
    io::{stdout, Write},
    thread, time,
};

use crossterm::{
    queue, style, AlternateScreen, Clear, ClearType, Color, Goto, Hide, Output, PrintStyledFont,
    Result, Show,
};

fn print_wait_screen() -> Result<()> {
    let mut stdout = stdout();
    queue!(stdout, Hide)?;
    queue!(stdout, Clear(ClearType::All))?;
    queue!(stdout, Goto(0, 0))?;
    queue!(
        stdout,
        Output(
            "Welcome to the wait screen.\n\
             Please wait a few seconds until we arrive back at the main screen.\n\
             Progress: "
                .to_string(),
        )
    )?;
    stdout.flush()?;
    // print some progress example.
    let items = 5;
    for i in 1..=items {
        // print the current counter at the line of `Seconds to Go: {counter}`
        queue!(stdout, Goto(10, 2))?;
        queue!(
            stdout,
            PrintStyledFont(
                style(format!("{} of the {} items processed", i, items))
                    .with(Color::Red)
                    .on(Color::Blue)
            )
        )?;
        stdout.flush()?;
        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
    queue!(stdout, Show)?; // we must restore the cursor
    Ok(())
}

/// print wait screen on alternate screen, then switch back.
fn print_wait_screen_on_alternate_window() -> Result<()> {
    let _alt = AlternateScreen::to_alternate(false)?;
    print_wait_screen()
}

// cargo run --bin alternate_screen_command
fn main() -> Result<()> {
    print_wait_screen_on_alternate_window()
}
