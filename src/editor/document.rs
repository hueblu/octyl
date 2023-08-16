use tui::widgets::Paragraph;

use crate::buffer::TextBuffer;

#[derive(Clone)]
pub struct Document {
    buf: TextBuffer,

    cursor: (u16, u16),
    scroll: (u16, u16),
}

impl<I> From<I> for Document
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(value: I) -> Self {
        Self {
            buf: TextBuffer::from(value),

            cursor: (0, 0),
            scroll: (0, 0),
        }
    }
}

impl Document {
    pub fn new() -> Self {
        Self {
            buf: TextBuffer::new(vec![]),
            cursor: (0, 0),
            scroll: (0, 0),
        }
    }

    pub fn insert_char<C: Into<char>>(&mut self, _c: C) {
        todo!()
    }

    pub fn delete_char(&mut self) {
        todo!()
    }

    pub fn insert_newline(&mut self) {
        todo!()
    }

    pub fn paragraph<'a>(&self) -> Paragraph<'a> {
        Paragraph::new(self.buf.text()).scroll(self.scroll)
    }
}
