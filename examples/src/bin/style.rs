//!
//! Examples of coloring the terminal.
//!

#![allow(dead_code)]

use crossterm::style::{available_color_count, SetForegroundColor};
use crossterm::style::{Attribute, Color, Colorize, Styler};

/// print some red text | demonstration.
fn paint_foreground() {
    println!("{}", "Red foreground text: {}".red());
}

/// print some text on red background | demonstration.
fn paint_background() {
    println!("{}", "Red background text: {}".on_red());
}

/// Print all available foreground colors | demonstration.
fn print_all_foreground_colors_with_method() {
    println!("Black : \t\t       {} {}\n", "■".black(), Attribute::Reset);
    println!(
        "DarkGrey : \t\t     {} {}\n",
        "■".dark_grey(),
        Attribute::Reset
    );
    println!("Red : \t\t         {} {}\n", "■".red(), Attribute::Reset);
    println!(
        "DarkRed : \t\t     {} {}\n",
        "■".dark_red(),
        Attribute::Reset
    );
    println!("Cyan : \t\t        {} {}\n", "■".cyan(), Attribute::Reset);
    println!(
        "DarkCyan : \t\t    {} {}\n",
        "■".dark_cyan(),
        Attribute::Reset
    );
    println!("Green : \t\t       {} {}\n", "■".green(), Attribute::Reset);
    println!(
        "DarkGreen : \t\t   {} {}\n",
        "■".dark_green(),
        Attribute::Reset
    );
    println!("Blue : \t\t        {} {}\n", "■".blue(), Attribute::Reset);
    println!(
        "DarkBlue : \t\t    {} {}\n",
        "■".dark_blue(),
        Attribute::Reset
    );
    println!(
        "Magenta : \t\t     {} {}\n",
        "■".magenta(),
        Attribute::Reset
    );
    println!(
        "DarkMagenta : \t\t {} {}\n",
        "■".dark_magenta(),
        Attribute::Reset
    );
    println!("Yellow : \t\t      {} {}\n", "■".yellow(), Attribute::Reset);
    println!(
        "DarkYellow : \t\t  {} {}\n",
        "■".dark_yellow(),
        Attribute::Reset
    );
    println!("Grey : \t\t        {} {}\n", "■".grey(), Attribute::Reset);
    println!("White : \t\t       {} {}\n", "■".white(), Attribute::Reset);
}

/// Print all available foreground colors | demonstration.
fn print_all_background_colors_with_method() {
    println!(
        "Black : \t\t       {} {}\n",
        "■".on_black(),
        Attribute::Reset
    );
    println!(
        "DarkGrey : \t\t       {} {}\n",
        "■".on_dark_grey(),
        Attribute::Reset
    );
    println!("Red : \t\t         {} {}\n", "■".on_red(), Attribute::Reset);
    println!(
        "DarkRed : \t\t     {} {}\n",
        "■".on_dark_red(),
        Attribute::Reset
    );
    println!(
        "Cyan : \t\t        {} {}\n",
        "■".on_cyan(),
        Attribute::Reset
    );
    println!(
        "DarkCyan : \t\t    {} {}\n",
        "■".on_dark_cyan(),
        Attribute::Reset
    );
    println!(
        "Green : \t\t       {} {}\n",
        "■".on_green(),
        Attribute::Reset
    );
    println!(
        "DarkGreen : \t\t   {} {}\n",
        "■".on_dark_green(),
        Attribute::Reset
    );
    println!(
        "Blue : \t\t        {} {}\n",
        "■".on_blue(),
        Attribute::Reset
    );
    println!(
        "DarkBlue : \t\t    {} {}\n",
        "■".on_dark_blue(),
        Attribute::Reset
    );
    println!(
        "Magenta : \t\t     {} {}\n",
        "■".on_magenta(),
        Attribute::Reset
    );
    println!(
        "DarkMagenta : \t\t {} {}\n",
        "■".on_dark_magenta(),
        Attribute::Reset
    );
    println!(
        "Yellow : \t\t      {} {}\n",
        "■".on_yellow(),
        Attribute::Reset
    );
    println!(
        "DarkYellow : \t\t  {} {}\n",
        "■".on_dark_yellow(),
        Attribute::Reset
    );
    println!(
        "Grey : \t\t        {} {}\n",
        "■".on_grey(),
        Attribute::Reset
    );
    println!(
        "White : \t\t       {} {}\n",
        "■".on_white(),
        Attribute::Reset
    );
}

/// Print text with all available attributes. Note that this can only be used at unix systems and that some are not supported widely | demonstration..
#[cfg(unix)]
fn print_text_with_attributes() {
    println!("{}", "Normal text");
    println!("{}", "Bold text".bold());
    println!("{}", "Italic text".italic());
    println!("{}", "Slow blinking text".slow_blink());
    println!("{}", "Rapid blinking text".rapid_blink());
    println!("{}", "Hidden text".hidden());
    println!("{}", "Underlined text".underlined());
    println!("{}", "Reversed text".reverse());
    println!("{}", "Dim text".dim());
    println!("{}", "Crossed out text".crossed_out());
    // ...

    println!(
        "{} Underlined {} No Underline",
        Attribute::Underlined,
        Attribute::NoUnderline
    );
    // ...
}

// Print text with all available attributes. Note that this can only be used at unix systems and that some are not supported widely | demonstration..
#[cfg(windows)]
fn print_text_with_attributes() {
    println!("{}", "Normal text");
    println!("{}", "Bold text".bold());
    println!("{}", "Underlined text".underlined());
    println!("{}", "Negative text".negative());
}

/// Print all supported RGB colors, not supported for Windows systems < 10  | demonstration.
fn print_supported_colors() {
    let count = available_color_count();

    for i in 0..count {
        println!("Test {}", SetForegroundColor(Color::AnsiValue(i as u8)));
    }
}

fn reset_fg_and_bg() {
    println!("{}", SetForegroundColor(Color::Reset));
    println!("{}", SetForegroundColor(Color::Reset));
}

// cargo run --example style
fn main() {
    print_all_background_colors_with_method()
}
