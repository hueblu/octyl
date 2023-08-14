mod document;

use document::Document;
use tui::{
    prelude::{Alignment, Buffer, Rect},
    style::Style,
    widgets::{Block, Widget},
};

#[derive(Clone)]
pub struct Editor<'a> {
    current_doc: usize,
    docs: Vec<Document>,

    block: Option<Block<'a>>,
    alignment: Alignment,
    style: Style,
}

impl<'a> Editor<'a> {
    pub fn new() -> Self {
        Self {
            current_doc: 0,
            docs: vec![Document::new()],

            block: None,
            alignment: Alignment::Left,
            style: Style::default(),
        }
    }

    fn get_current_document(&self) -> &Document {
        &self.docs[self.current_doc]
    }
}

impl<'a> Default for Editor<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, I> From<I> for Editor<'a>
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(value: I) -> Self {
        let mut editor = Self::new();
        editor.docs = vec![Document::from(value)];
        editor
    }
}

impl<'a> Widget for Editor<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut inner = self
            .get_current_document()
            .paragraph()
            .style(self.style)
            .alignment(self.alignment);

        if let Some(b) = self.block {
            inner = inner.block(b);
        }

        inner.render(area, buf);
    }
}
