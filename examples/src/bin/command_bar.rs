use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crossterm::cursor::{Hide, Show};
use crossterm::{
    cursor::MoveTo,
    input::{input, InputEvent, KeyEvent},
    screen::RawScreen,
    terminal::{self, Clear, ClearType},
    ExecutableCommand, Output, Result,
};

fn log(input_buf: Arc<Mutex<String>>) -> Result<Vec<thread::JoinHandle<()>>> {
    let mut threads = Vec::with_capacity(10);

    let (_, term_height) = terminal::size()?;

    for i in 0..1 {
        let input_buffer = input_buf.clone();

        let join = thread::spawn(move || {
            for j in 0..1000 {
                if let Err(_) = swap_write(
                    format!("Some output: {} from thread: {}", j, i).as_ref(),
                    &input_buffer.lock().unwrap(),
                    term_height,
                ) {
                    return;
                }
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        threads.push(join);
    }

    Ok(threads)
}

fn swap_write(msg: &str, input_buf: &String, term_height: u16) -> Result<()> {
    stdout()
        .execute(MoveTo(0, term_height))?
        .execute(Clear(ClearType::CurrentLine))?
        .execute(Output(format!("{}\r\n", msg)))?
        .execute(Output(format!("> {}", input_buf)))?;

    Ok(())
}

// cargo run --example command_bar
fn main() -> Result<()> {
    let _screen = RawScreen::into_raw_mode();
    stdout().execute(Hide)?;

    let input_buf = Arc::new(Mutex::new(String::new()));

    let threads = log(input_buf.clone())?;

    let mut count = 0;

    thread::spawn(move || {
        let input = input();
        let mut stdin = input.read_async();

        loop {
            match stdin.next() {
                Some(InputEvent::Keyboard(KeyEvent::Enter)) => {
                    input_buf.lock().unwrap().clear();
                }
                Some(InputEvent::Keyboard(KeyEvent::Char(character))) => {
                    input_buf.lock().unwrap().push(character as char);
                }
                _ => {}
            }

            thread::sleep(time::Duration::from_millis(10));
            count += 1;
        }
    })
    .join()
    .expect("Couldn't join");

    for thread in threads {
        thread.join().expect("Couldn't join");
    }

    stdout().execute(Show)?;

    Ok(())
}
