use std::io::stdout;

use crossterm::{cursor::MoveTo, execute};

use crate::error::Result;

pub struct ScreenBuffer {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl ScreenBuffer {
    pub fn new() -> Self {
        let (width, height) = crossterm::terminal::size().unwrap().into();
        let data = vec![' '; (width * height) as usize];
        Self {
            width: width.into(),
            height: height.into(),
            data,
        }
    }

    pub fn render(&self) -> Result<()> {
        execute!(stdout(), MoveTo(0, 0))?;
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.data[(y * self.width + x) as usize];
                print!("{c}");
            }
        }
        Ok(())
    }

    pub fn write_char_at_idx(&mut self, c: char, idx: usize) -> Result<()> {
        self.data[idx as usize] = c;
        Ok(())
    }

    pub fn write_char_at_coords(&mut self, c: char, x: usize, y: usize) -> Result<()> {
        self.data[(y * self.width + x) as usize] = c;
        Ok(())
    }

    pub fn write_char_at_cursor(&mut self, c: char) -> Result<()> {
        let (x, y) = crossterm::cursor::position()?;
        self.write_char_at_coords(c, x.into(), y.into())?;
        Ok(())
    }
}
