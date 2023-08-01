use std::{any::Any, io::stdout};

use crossterm::{cursor::MoveTo, execute};

use crate::{error::Result, math::Rect};

pub trait Component: Any {
    fn render(&mut self, size: (usize, usize)) -> Result<CharBuffer>;
    fn resize(&mut self, size: (usize, usize)) -> Result<()>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

pub struct Compositor {
    buffers: [CharBuffer; 2],
    active_buffer: usize,
}

impl Compositor {
    pub fn new() -> Self {
        let screen_size = crossterm::terminal::size().unwrap();
        let screen_size = (screen_size.0 as usize, screen_size.1 as usize);

        let buffers = [
            CharBuffer::new(screen_size.0, screen_size.1),
            CharBuffer::new(screen_size.0, screen_size.1),
        ];

        Self {
            buffers,
            active_buffer: 0,
        }
    }

    /// renders the active buffer to the screen
    /// and switches the active buffer
    pub fn render(&mut self) -> Result<()> {
        //TODO: find the diff between the two buffers
        // and only render the diff

        self.buffers[self.active_buffer].render()?;
        self.switch_active_buffer();
        Ok(())
    }

    fn switch_active_buffer(&mut self) {
        self.active_buffer = (self.active_buffer + 1) % 2;
    }

    pub fn draw(&mut self, f: impl FnOnce(&mut Frame)) {
        let mut frame = Frame {
            buffer: &mut self.buffers[self.active_buffer],
            components: vec![],
        };

        f(&mut frame);
    }
}

// given to user to manipulate
// during drawing closure
pub struct Frame<'a> {
    buffer: &'a mut CharBuffer,
    components: Vec<(Box<dyn Component>, Rect)>,
}

impl<'a> Frame<'a> {
    pub fn add_component(&mut self, component: impl Component, rect: Rect) {
        self.components.push((Box::new(component), rect));
    }
}

#[derive(Debug, Clone)]
pub struct CharBuffer {
    buffer_size: (usize, usize),
    data: Vec<char>,
}

impl CharBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![' '; (width * height) as usize];
        Self {
            buffer_size: (width, height),
            data,
        }
    }

    pub fn render(&self) -> Result<()> {
        execute!(stdout(), MoveTo(0, 0))?;
        for y in 0..self.buffer_size.1 {
            for x in 0..self.buffer_size.0 {
                let c = self.data[(y * self.buffer_size.0 + x) as usize];
                print!("{c}");
            }
        }
        Ok(())
    }

    pub fn set_char_idx(&mut self, c: char, idx: usize) -> Result<()> {
        self.data[idx as usize] = c;
        Ok(())
    }

    pub fn set_char(&mut self, c: char, x: usize, y: usize) -> Result<()> {
        self.data[(y * self.buffer_size.0 + x) as usize] = c;
        Ok(())
    }
}
