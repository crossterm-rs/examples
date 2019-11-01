//! This is the same than alternate_screen but using
//! the command API instead of the "old style" direct
//! unbuffered API.

use std::{
    io::{stdout, Write},
    thread, time,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen},
    style::{style, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
    Output, Result,
};

/// manage the wait screen, using the provided Writer, which
/// may typically be stdout or stderr.
fn print_wait_screen<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(w, EnterAlternateScreen)?;
    queue!(w, Hide)?;
    queue!(w, Clear(ClearType::All))?;
    queue!(w, MoveTo(0, 0))?;
    queue!(
        w,
        Output(
            "Welcome to the wait screen.\n\
             Please wait a few seconds until we quit the application.\n\
             Progress: "
                .to_string(),
        )
    )?;
    w.flush()?;
    // print some progress example.
    let items = 5;
    for i in 1..=items {
        // print the current counter at the line of `Seconds to Go: {counter}`
        queue!(w, MoveTo(10, 2))?;
        queue!(
            w,
            PrintStyledContent(
                style(format!("{} of the {} items processed", i, items))
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
    w.flush()?;
    Ok(())
}

// Run with:
//    cargo run --bin alternate_screen_command
fn main() {
    let mut stdout = stdout();
    print_wait_screen(&mut stdout).unwrap();
}
