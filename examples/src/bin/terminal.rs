//!
//! Terminal Examples
//!

#![allow(dead_code)]

use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{self, Clear, ClearType, ScrollDown, ScrollUp, SetSize},
    Result,
};

fn print_test_data() {
    for i in 0..100 {
        println!("Test data to test terminal: {}", i);
    }
}

/// Clear all lines in terminal | demonstration
fn clear_all_lines() -> Result<()> {
    print_test_data();

    // Clear all lines in terminal;
    execute!(stdout(), Clear(ClearType::All))
}

/// Clear all lines from cursor position X:4, Y:4 down | demonstration
fn clear_from_cursor_down() -> Result<()> {
    print_test_data();

    // Clear all cells from current cursor position down.
    execute!(stdout(), MoveTo(4, 8), Clear(ClearType::FromCursorDown))
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
fn clear_from_cursor_up() -> Result<()> {
    print_test_data();

    // Clear all cells from current cursor position down.
    execute!(stdout(), MoveTo(4, 4), Clear(ClearType::FromCursorUp))
}

/// Clear all lines from cursor position X:4, Y:4 up | demonstration
fn clear_current_line() -> Result<()> {
    print_test_data();

    // Clear current line cells.
    execute!(stdout(), MoveTo(4, 3), Clear(ClearType::CurrentLine))
}

/// Clear all lines from cursor position X:4, Y:7 up | demonstration
fn clear_until_new_line() -> Result<()> {
    print_test_data();

    // Clear all the cells until next line.
    execute!(stdout(), MoveTo(4, 20), Clear(ClearType::UntilNewLine))
}

/// Print the the current terminal size | demonstration.
fn print_terminal_size() -> Result<()> {
    // Get terminal size
    let (width, height) = terminal::size()?;

    // Print results
    print!("X: {}, y: {}", width, height);
    Ok(())
}

/// Set the terminal size to width 10, height: 10 | demonstration.
fn set_terminal_size() -> Result<()> {
    execute!(stdout(), SetSize(4, 20))
}

/// Scroll down 10 lines | demonstration.
fn scroll_down() -> Result<()> {
    print_test_data();
    // Scroll down 10 lines.
    execute!(stdout(), ScrollDown(10))
}

/// Scroll down 10 lines | demonstration.
fn scroll_up() -> Result<()> {
    print_test_data();

    // Scroll up 10 lines.
    execute!(stdout(), ScrollUp(10))
}

/// exit the current proccess.
fn exit() {
    terminal::exit();
}

// cargo run --example terminal
fn main() -> Result<()> {
    scroll_down()
}
