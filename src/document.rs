#![allow(dead_code)]

use ropey::RopeSlice;

use crate::compositor::{CharBuffer, Component};
use crate::error::Result;

pub struct Document {
    rope: ropey::Rope,

    // (x, y)
    screen_size: (usize, usize),
    line_wrap: bool,

    scroll: usize,
    cursor: Cursor,
}

impl Document {
    pub fn new(screen_size: (usize, usize)) -> Self {
        Self {
            rope: ropey::Rope::new(),

            screen_size,
            line_wrap: false,

            scroll: 0,
            cursor: Cursor::new(),
        }
    }

    pub fn insert_char<C: Into<char>>(&mut self, char: C) {
        let char = char.into();
        self.rope.insert_char(self.get_cursor_idx(), char);

        if char == '\n' {
            self.cursor.current_pos.0 = 0;
            self.cursor.current_pos.1 += 1;
        } else {
            self.cursor.current_pos.0 += 1;
        }
    }

    pub fn insert_string<S: ToString>(&mut self, string: S) {
        self.rope
            .insert(self.cursor.current_pos.0, string.to_string().as_str());
        self.cursor.current_pos.0 += string.to_string().len();
    }

    pub fn cursor_coords(&self) -> (usize, usize) {
        self.cursor.current_pos
    }

    // moves the cursor the specified amount,
    // wrapping around lines if necessary
    pub fn move_cursor(&mut self, x: isize, y: isize) {
        if x.is_positive() {
            for _ in 0..x {
                self.move_cursor_right();
            }
        } else {
            for _ in 0..x.abs() {
                self.move_cursor_left();
            }
        }

        if y.is_positive() {
            for _ in 0..y {
                self.move_cursor_down();
            }
        } else {
            for _ in 0..y.abs() {
                self.move_cursor_up();
            }
        }
    }

    pub fn move_cursor_down(&mut self) {}

    pub fn move_cursor_up(&mut self) {
        todo!()
    }

    pub fn move_cursor_left(&mut self) {
        todo!()
    }

    pub fn move_cursor_right(&mut self) {
        todo!()
    }

    pub fn resize(&mut self, screen_size: (usize, usize)) {
        self.screen_size = screen_size;
    }

    /// Returns the lines in the buffer, splitting lines that are longer than
    /// `max_line_len`.
    fn lines_capped(&self) -> Vec<RopeSlice<'_>> {
        let mut lines: Vec<RopeSlice<'_>> = Vec::new();

        if self.line_wrap {
            for line in self.rope.lines() {
                let mut start_pos = 0;

                while start_pos < line.len_chars() {
                    let end_pos = std::cmp::min(start_pos + self.screen_size.0, line.len_chars());
                    lines.push(line.slice(start_pos..end_pos));
                    start_pos = end_pos;
                }
            }
        } else {
            return self.rope.lines().collect();
        }

        lines
    }

    fn get_cursor_idx(&self) -> usize {
        let mut chars = self.cursor.current_pos.0;

        self.rope
            .lines()
            .take(self.cursor.current_pos.1)
            .for_each(|line| {
                chars += line.len_chars();
            });

        chars
    }
}

impl Component for Document {
    fn render(&mut self, size: (usize, usize)) -> Result<CharBuffer> {
        self.resize(size);
        let mut buff = CharBuffer::new(size.0, size.1);

        for (y, line) in self
            .lines_capped()
            .into_iter()
            .skip(self.scroll)
            .enumerate()
        {
            for (x, c) in line.chars().enumerate() {
                buff.set_char(c, x, y)?;
            }
        }

        Ok(buff)
    }

    fn resize(&mut self, size: (usize, usize)) -> Result<()> {
        self.resize(size);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

/// Represents the cursor in the document
struct Cursor {
    // position of the cursor in the document
    // that shows on the screen
    current_pos: (usize, usize),

    // x position that the cursor
    // should maximize to when moving
    // to longer lines
    actual_x_pos: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    fn new() -> Self {
        Self {
            current_pos: (0, 0),
            actual_x_pos: 0,
        }
    }
}
