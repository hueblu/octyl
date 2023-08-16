mod document;

use document::Document;
use tui::{
    prelude::{Alignment, Buffer, Rect},
    style::Style,
    widgets::{Block, Widget},
};

use crossterm::event::{Event as CrossEvent, KeyCode, KeyEvent};

#[derive(Clone)]
pub struct Editor<'a> {
    current_doc: usize,
    docs: Vec<Document>,
    mode: Mode,

    block: Option<Block<'a>>,
    alignment: Alignment,
    style: Style,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
}

impl<'a> Editor<'a> {
    pub fn new() -> Self {
        Self {
            current_doc: 0,
            docs: vec![Document::new()],
            mode: Mode::Normal,

            block: None,
            alignment: Alignment::Left,
            style: Style::default(),
        }
    }

    fn get_doc(&self) -> &Document {
        &self.docs[self.current_doc]
    }

    fn get_doc_mut(&mut self) -> &mut Document {
        &mut self.docs[self.current_doc]
    }

    pub fn handle_terminal_event(&mut self, event: CrossEvent) {
        match self.mode {
            Mode::Insert => {
                if let CrossEvent::Key(KeyEvent { code, .. }) = event {
                    match code {
                        KeyCode::Esc => self.mode = Mode::Normal,
                        KeyCode::Char(c) => self.get_doc_mut().insert_char::<char>(c.into()),
                        KeyCode::Backspace => self.get_doc_mut().delete_char(),
                        KeyCode::Enter => self.get_doc_mut().insert_newline(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
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
            .get_doc()
            .paragraph()
            .style(self.style)
            .alignment(self.alignment);

        if let Some(b) = self.block {
            inner = inner.block(b);
        }

        inner.render(area, buf);
    }
}
