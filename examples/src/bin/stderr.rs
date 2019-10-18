//! This shows how an application can write on stderr
//! instead of stdout, thus making it possible to
//! the command API instead of the "old style" direct
//! unbuffered API.
//!
//! This particular example is only suited to Unix
//! for now.

use std::io::{stderr, Write};

use crossterm::{
    input, queue, EnterAlternateScreen, Goto, Hide, LeaveAlternateScreen, Output, RawScreen,
    Result, Show,
};

const TEXT: &str = r#"
This screen is ran on stderr.
And when you hit enter, it prints on stdout.
This makes it possible to run an application and choose what will
be sent to any application calling yours.

For example, assuming you build this example with

    cargo build --bin stderr

and then you run it with

    cd "$(target/debug/stderr)"

what the application prints on stdout is used as argument to cd.

Try it out.

Hit any key to quit this screen:

1 will print `..`
2 will print `/`
3 will print `~`
Any other key will print this text (so that you may copy-paste)
"#;

fn run_app<W>(w: &mut W) -> Result<char>
where
    W: Write,
{
    queue!(w, EnterAlternateScreen)?;
    queue!(w, Hide)?; // hiding the cursor
    let mut y = 1;
    for line in TEXT.split('\n') {
        queue!(w, Goto(1, y))?;
        queue!(w, Output(line.to_string()))?;
        y += 1;
    }
    w.flush()?;
    let _raw = RawScreen::into_raw_mode()?;
    let user_char = input().read_char()?; // we wait for the user to hit a key
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(user_char)
}

// cargo run --bin stderr
fn main() {
    match run_app(&mut stderr()).unwrap() {
        '1' => print!(".."),
        '2' => print!("/"),
        '3' => print!("~"),
        _ => println!("{}", TEXT),
    }
}
