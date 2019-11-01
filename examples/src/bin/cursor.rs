//!
//! Examples of actions that could be performed with the cursor.
//!

#![allow(dead_code)]

use crossterm::{
    cursor::{
        self, DisableBlinking, EnableBlinking, Hide, MoveDown, MoveLeft, MoveRight, MoveTo, MoveUp,
        RestorePosition, SavePosition, Show,
    },
    execute, Result,
};
use std::io::{stdout, Write};

/// Set the cursor to position X: 10, Y: 5 in the terminal.
fn goto() -> Result<()> {
    // Set the cursor to position X: 10, Y: 5 in the terminal
    execute!(stdout(), MoveTo(10, 5))?;
    Ok(())
}

/// get the cursor position
fn pos() -> Result<()> {
    // get the cursor position.
    let (x, y) = cursor::position()?;
    println!("{} {}", x, y);
    Ok(())
}

/// Move the cursor 3 up | demonstration.
fn move_up() -> Result<()> {
    // Move the cursor to position 3 times to the up in the terminal
    execute!(stdout(), MoveUp(3))?;
    Ok(())
}

/// Move the cursor 3 down | demonstration.
fn move_down() -> Result<()> {
    // Move the cursor to position 3 times to the down in the terminal
    execute!(stdout(), MoveDown(3))?;
    Ok(())
}

/// Move the cursor 3 to the right | demonstration.
fn move_right() -> Result<()> {
    // Move the cursor to position 3 times to the right in the terminal
    execute!(stdout(), MoveRight(3))?;
    Ok(())
}

/// Move the cursor 3 left | demonstration.
fn move_left() -> Result<()> {
    // Move the cursor to position 3 times to the left in the terminal
    execute!(stdout(), MoveLeft(3))?;
    Ok(())
}

/// Save and reset cursor position | demonstration..
fn save_and_restore_position() -> Result<()> {
    // 1. MoveTo X: 5 Y: 5,
    // 2. Safe cursor position: X: 5 Y: 5,
    // 3. MoveTo X: 5 Y: 20
    execute!(stdout(), MoveTo(5, 5), SavePosition, MoveTo(5, 20))?;

    // Print at X: 5 Y: 20.
    println!("Yea!");
    // Reset back to X: 5 Y: 5.
    execute!(stdout(), RestorePosition)?;
    // Print Back at X: 5 Y: 5.
    println!("Back");
    Ok(())
}

/// Hide cursor display | demonstration.
fn hide_cursor() -> Result<()> {
    execute!(stdout(), Hide)?;
    Ok(())
}

/// Show cursor display | demonstration.
fn show_cursor() -> Result<()> {
    execute!(stdout(), Show)?;
    Ok(())
}

/// Show cursor display, only works on certain terminals.| demonstration
fn blink_cursor() -> Result<()> {
    println!("Going to enable blinking cursor & sleep for 5s...");
    execute!(stdout(), EnableBlinking)?;
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Going to disable blinking cursor...");
    execute!(stdout(), DisableBlinking)?;
    Ok(())
}

// cargo run --example cursor
fn main() -> Result<()> {
    blink_cursor()
}
