#![allow(dead_code)]
use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

#[derive(Debug)]
enum Error {
    Crossterm(crossterm::ErrorKind),
}

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    init_terminal()?;

    let mut close = false;
    let mut buff = ScreenBuffer::new();
    let (mut width, mut height) = crossterm::terminal::size()?;
    let mut cursor_pos = (0, 0);

    while !close {
        execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1))?;
        match read()? {
            Event::Key(keyevent) => match keyevent.code {
                KeyCode::Esc => close = true,
                KeyCode::Char(c) => {
                    buff.write_char_at_cursor(c)?;

                    cursor_pos.0 += 1;
                    if cursor_pos.0 > width {
                        cursor_pos.0 = 0;
                        cursor_pos.1 += 1;
                    }
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                (width, height) = (w, h);
            }
            _ => {}
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        buff.render()?;
    }

    cleanup_terminal()?;
    Ok(())
}

struct ScreenBuffer {
    width: u16,
    height: u16,
    data: Vec<char>,
}

impl ScreenBuffer {
    fn new() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap();
        let data = vec![' '; (width * height) as usize];
        Self {
            width,
            height,
            data,
        }
    }

    fn render(&self) -> Result<()> {
        execute!(stdout(), MoveTo(0, 0))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.data[(y * self.width + x) as usize];
                print!("{c}");
            }
        }
        Ok(())
    }

    fn write_char_at_idx(&mut self, c: char, idx: u16) -> Result<()> {
        self.data[idx as usize] = c;
        Ok(())
    }

    fn write_char_at_coords(&mut self, c: char, x: u16, y: u16) -> Result<()> {
        self.data[(y * self.width + x) as usize] = c;
        Ok(())
    }

    fn write_char_at_cursor(&mut self, c: char) -> Result<()> {
        let (x, y) = crossterm::cursor::position()?;
        self.write_char_at_coords(c, x, y)?;
        Ok(())
    }
}

fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All))?;
    Ok(())
}

fn cleanup_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, Clear(ClearType::All))?;
    Ok(())
}

impl From<crossterm::ErrorKind> for Error {
    fn from(error: crossterm::ErrorKind) -> Self {
        Error::Crossterm(error)
    }
}
