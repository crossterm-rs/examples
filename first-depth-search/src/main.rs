use std::io::{stdout, Write};
use std::iter::Iterator;
use std::{thread, time};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    input::input,
    input::{InputEvent, KeyEvent},
    screen::{AlternateScreen, RawScreen},
    style::{style, Color, PrintStyledContent, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, SetSize},
    Output, Result,
};

use self::variables::{Position, Size};
use crossterm::style::ResetColor;

mod algorithm;
mod map;
mod messages;
mod variables;

fn main() -> Result<()> {
    run()
}

/// run the program
fn run() -> Result<()> {
    //    let screen = RawScreen::into_raw_mode()?;
    print_welcome_screen()?;
    start_algorithm()?;
    exit()
}

fn start_algorithm() -> Result<()> {
    // we first want to switch to alternate screen. On the alternate screen we are going to run or firstdepthsearch algorithm
    let ref _alternate_screen = AlternateScreen::to_alternate(true)?;
    // setup the map size and the position to start searching for a path.
    let map_size = Size::new(50, 40);
    let start_pos = Position::new(10, 10);

    // create and render the map. Or map border is going to have an █ look and inside the map is just a space.
    let mut map = map::Map::new(map_size, '█', ' ');
    map.render_map()?;

    // create the algorithm and start it on the alternate screen. Make sure to pass the refrence to the AlternateScreen screen.
    let mut algorithm = algorithm::FirstDepthSearch::new(map, start_pos);
    algorithm.start()
}

fn print_welcome_screen() -> Result<()> {
    // we have to keep this screen arround to prevent te
    let _screen = RawScreen::into_raw_mode()?;

    execute!(
        stdout(),
        SetSize(110, 60),
        Clear(ClearType::All),
        MoveTo(0, 0),
        PrintStyledContent(
            style(format!("{}", messages::WELCOME_MESSAGE.join("\n\r"))).with(Color::Cyan)
        ),
        Hide,
        MoveTo(0, 10),
        Output("The first depth search algorithm will start in:   Seconds".to_string()),
        MoveTo(0, 11),
        Output("Press `q` to abort the program".to_string())
    )?;

    let mut stdin = input().read_async();

    // print some progress example.
    for i in (1..5).rev() {
        if let Some(InputEvent::Keyboard(KeyEvent::Char('q'))) = stdin.next() {
            exit()?;
            terminal::exit();
            break;
        } else {
            // print the current counter at the line of `Seconds to Go: {counter}`
            execute!(
                stdout(),
                MoveTo(48, 10),
                SetForegroundColor(Color::Red),
                SetBackgroundColor(Color::Blue),
                Output(i.to_string())
            )?;
        }

        execute!(stdout(), ResetColor)?;

        // 1 second delay
        thread::sleep(time::Duration::from_secs(1));
    }

    Ok(())
}

fn exit() -> Result<()> {
    RawScreen::disable_raw_mode()?;
    execute!(stdout(), Show, ResetColor)?;
    Ok(())
}
