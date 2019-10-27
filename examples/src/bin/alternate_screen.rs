use std::{
    io::{stdout, Write},
    thread, time,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, style,
    terminal::{Clear, ClearType},
    utils::Output,
    AlternateScreen, Color, PrintStyledFont, Result,
};

fn print_wait_screen() -> Result<()> {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Hide,
        Output(
            "Welcome to the wait screen.\n\
             Please wait a few seconds until we arrive back at the main screen.\n\
             Progress: "
                .to_string()
        )
    )?;

    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        execute!(
            stdout(),
            MoveTo(10, 2),
            PrintStyledFont(
                style(format!("{} of the 5 items processed", i))
                    .with(Color::Red)
                    .on(Color::Blue)
            )
        )?;

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
    execute!(stdout(), Show)?; // we must restore the cursor
    Ok(())
}

/// print wait screen on alternate screen, then switch back.
fn print_wait_screen_on_alternate_window() -> Result<()> {
    let _alt = AlternateScreen::to_alternate(false)?;
    print_wait_screen()
}

// cargo run --bin alternate_screen
fn main() -> Result<()> {
    print_wait_screen_on_alternate_window()
}
