use tui::widgets::Paragraph;

use crate::buffer::TextBuffer;

#[derive(Clone)]
pub struct Document {
    buf: TextBuffer,

    cursor: Cursor,
    scroll: (u16, u16),
}

#[derive(Clone, Copy, Default, Debug)]
pub(super) struct Cursor {
    x: u16,
    y: u16,
}

impl<I> From<I> for Document
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(value: I) -> Self {
        Self {
            buf: TextBuffer::from(value),

            cursor: Cursor::default(),
            scroll: (0, 0),
        }
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            buf: TextBuffer::new(vec![]),
            cursor: Cursor::default(),
            scroll: (0, 0),
        }
    }

    pub async fn insert_char<C: Into<char>>(&mut self, c: C) {
        self.buf.insert_char(self.cursor.into(), c.into());
    }

    pub async fn delete_char(&mut self) {
        self.buf.delete_char(self.cursor.into());
    }

    pub async fn insert_newline(&mut self) {}

    pub fn paragraph<'a>(&self) -> Paragraph<'a> {
        Paragraph::new(self.buf.text()).scroll(self.scroll)
    }

    pub fn cursor_coords(&self) -> (u16, u16) {
        self.cursor.into()
    }
}

impl From<(u16, u16)> for Cursor {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Into<(u16, u16)> for Cursor {
    fn into(self) -> (u16, u16) {
        (self.x, self.y)
    }
}
