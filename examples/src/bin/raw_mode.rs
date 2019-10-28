use std::io::{stdout, Write};
use std::{thread, time};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, style,
    terminal::{Clear, ClearType},
    utils::Output,
    AlternateScreen, Color, PrintStyledFont, Result,
};

fn print_wait_screen() -> Result<()> {
    let mut stdout = stdout();

    execute!(
        &mut stdout,
        Clear(ClearType::All),
        Hide,
        MoveTo(0, 0),
        Output("Welcome to the wait screen.".to_string()),
        MoveTo(0, 1),
        Output("Please wait a few seconds until we arrive back at the main screen.".to_string()),
        MoveTo(0, 2),
        Output("Progress:".to_string()),
        MoveTo(0, 3)
    )?;

    // print some progress example.
    for i in 1..5 {
        // print the current counter at the line of `Seconds to Go: {counter}`
        execute!(
            &mut stdout,
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

    execute!(stdout, Show)?; // lets restore the cursor
    Ok(())
}

fn print_wait_screen_on_alternate_window() -> Result<()> {
    // by passing in 'true' the alternate screen will be in raw modes.
    let _alt = AlternateScreen::to_alternate(true)?;
    print_wait_screen()
}

// cargo run --example raw_mode
fn main() -> Result<()> {
    print_wait_screen_on_alternate_window()
}
