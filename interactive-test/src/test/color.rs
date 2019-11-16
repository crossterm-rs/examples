use crate::{MoveCursorToNextLine, Result};
use crossterm::{cursor, queue, style, style::Color, Output};
use std::io::Write;

const COLORS: [Color; 21] = [
    Color::Black,
    Color::DarkGrey,
    Color::Grey,
    Color::White,
    Color::DarkRed,
    Color::Red,
    Color::DarkGreen,
    Color::Green,
    Color::DarkYellow,
    Color::Yellow,
    Color::DarkBlue,
    Color::Blue,
    Color::DarkMagenta,
    Color::Magenta,
    Color::DarkCyan,
    Color::Cyan,
    Color::AnsiValue(0),
    Color::AnsiValue(15),
    Color::Rgb { r: 255, g: 0, b: 0 },
    Color::Rgb { r: 0, g: 255, b: 0 },
    Color::Rgb { r: 0, g: 0, b: 255 },
];

fn test_set_foreground_color<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(
        w,
        Output("Foreground colors on the black & white background"),
        MoveCursorToNextLine(2)
    )?;

    for color in &COLORS {
        queue!(
            w,
            style::SetForegroundColor(*color),
            style::SetBackgroundColor(Color::Black),
            Output(format!(
                "{:>width$} ",
                format!("{:?} ████████████", color),
                width = 40
            )),
            style::SetBackgroundColor(Color::White),
            Output(format!(
                "{:>width$}",
                format!("{:?} ████████████", color),
                width = 40
            )),
            MoveCursorToNextLine(1)
        )?;
    }

    w.flush()?;

    Ok(())
}

fn test_set_background_color<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    queue!(
        w,
        Output("Background colors with black & white foreground"),
        MoveCursorToNextLine(2)
    )?;

    for color in &COLORS {
        queue!(
            w,
            style::SetBackgroundColor(*color),
            style::SetForegroundColor(Color::Black),
            Output(format!(
                "{:>width$} ",
                format!("{:?} ▒▒▒▒▒▒▒▒▒▒▒▒", color),
                width = 40
            )),
            style::SetForegroundColor(Color::White),
            Output(format!(
                "{:>width$}",
                format!("{:?} ▒▒▒▒▒▒▒▒▒▒▒▒", color),
                width = 40
            )),
            MoveCursorToNextLine(1)
        )?;
    }

    w.flush()?;

    Ok(())
}

fn test_color_values_matrix_16x16<W, F>(w: &mut W, title: &str, color: F) -> Result<()>
where
    W: Write,
    F: Fn(u16, u16) -> Color,
{
    queue!(w, Output(title))?;

    for idx in 0..=15 {
        queue!(
            w,
            cursor::MoveTo(1, idx + 4),
            Output(format!("{:>width$}", idx, width = 2))
        )?;
        queue!(
            w,
            cursor::MoveTo(idx * 3 + 3, 3),
            Output(format!("{:>width$}", idx, width = 3))
        )?;
    }

    for row in 0..=15u16 {
        queue!(w, cursor::MoveTo(4, row + 4))?;
        for col in 0..=15u16 {
            queue!(w, style::SetForegroundColor(color(col, row)), Output("███"))?;
        }
        queue!(
            w,
            style::SetForegroundColor(Color::White),
            Output(format!("{:>width$} ..= ", row * 16, width = 3)),
            Output(format!("{:>width$}", row * 16 + 15, width = 3))
        )?;
    }

    w.flush()?;

    Ok(())
}

fn test_color_ansi_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Ansi values", |col, row| {
        Color::AnsiValue((row * 16 + col) as u8)
    })
}

fn test_rgb_red_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Rgb red values", |col, row| Color::Rgb {
        r: (row * 16 + col) as u8,
        g: 0 as u8,
        b: 0,
    })
}

fn test_rgb_green_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Rgb green values", |col, row| Color::Rgb {
        r: 0,
        g: (row * 16 + col) as u8,
        b: 0,
    })
}

fn test_rgb_blue_values<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    test_color_values_matrix_16x16(w, "Color::Rgb blue values", |col, row| Color::Rgb {
        r: 0,
        g: 0,
        b: (row * 16 + col) as u8,
    })
}

#[allow(clippy::cognitive_complexity)]
pub fn run<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    run_tests!(
        w,
        test_set_foreground_color,
        test_set_background_color,
        test_color_ansi_values,
        test_rgb_red_values,
        test_rgb_green_values,
        test_rgb_blue_values,
    );
    Ok(())
}
