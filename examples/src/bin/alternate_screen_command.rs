//! This is the same than alternate_screen but using
//! the command API instead of the "old style" direct
//! unbuffered API.

use std::{
    io::{stdout, Write},
    thread, time,
};

use crossterm::{
    queue, style, Clear, ClearType, Color, EnterAlternateScreen, Goto, Hide, LeaveAlternateScreen,
    Output, PrintStyledFont, Result, Show,
};

fn print_wait_screen<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(w, EnterAlternateScreen)?;
    queue!(w, Hide)?;
    queue!(w, Clear(ClearType::All))?;
    queue!(w, Goto(0, 0))?;
    queue!(
        w,
        Output(
            "Welcome to the wait screen.\n\
             Please wait a few seconds until we arrive back at the main screen.\n\
             Progress: "
                .to_string(),
        )
    )?;
    w.flush()?;
    // print some progress example.
    let items = 5;
    for i in 1..=items {
        // print the current counter at the line of `Seconds to Go: {counter}`
        queue!(w, Goto(10, 2))?;
        queue!(
            w,
            PrintStyledFont(
                style(format!("{} of the {} bip items processed", i, items))
                    .with(Color::Red)
                    .on(Color::Blue)
            )
        )?;
        w.flush()?;
        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    Ok(())
}

/// print wait screen on alternate screen, then switch back.
fn print_wait_screen_on_alternate_window() -> Result<()> {
    let mut stdout = stdout();
    // note that we might have used stderr instead of stdout here
    print_wait_screen(&mut stdout)
}

// cargo run --bin alternate_screen_command
fn main() {
    print_wait_screen_on_alternate_window().unwrap();
}
