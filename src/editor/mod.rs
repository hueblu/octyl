mod document;

use anyhow::Result;

use document::Document;
use tui::{
    prelude::{Alignment, Buffer, Rect},
    style::Style,
    widgets::{Block, Widget},
};

use crossterm::event::{Event as CrossEvent, KeyCode, KeyEvent, KeyEventKind};

use crate::app::Component;

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
pub enum Mode {
    Normal,
    Insert,
}

impl<'a> Editor<'a> {
    pub fn new() -> Self {
        Self {
            current_doc: 0,
            docs: vec![Document::new()],
            mode: Mode::Insert,

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

    pub async fn handle_terminal_event(&mut self, event: CrossEvent) -> Result<()> {
        if let CrossEvent::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            use KeyCode::*;

            match self.mode {
                Mode::Insert => match code {
                    Esc => self.mode = Mode::Normal,
                    Char(c) => self.get_doc_mut().insert_char::<char>(c.into()).await,
                    Backspace => self.get_doc_mut().delete_char().await,
                    Enter => self.get_doc_mut().insert_newline().await,
                    _ => {}
                },
                Mode::Normal => match code {
                    Char(c) => match c {
                        'i' => self.mode = Mode::Insert,
                        _ => {}
                    },
                    _ => {}
                },
            }
        }
        Ok(())
    }
}

pub struct EditorRenderer<'a> {
    doc: &'a Document,
    style: Style,
    alignment: Alignment,
    block: Option<Block<'a>>,
}

impl<'a> Component<'a> for Editor<'a> {
    type Renderer = EditorRenderer<'a>;

    fn as_widget(&'a self) -> Self::Renderer {
        EditorRenderer::from(self)
    }

    fn get_cursor(&self) -> Option<(u16, u16)> {
        log::info!(
            "Getting cursor coords at {:?}",
            self.get_doc().cursor_coords()
        );
        Some(self.get_doc().cursor_coords())
    }
}

impl<'a> EditorRenderer<'a> {
    fn from<'b>(editor: &'b Editor) -> Self
    where
        'b: 'a,
    {
        Self {
            doc: editor.get_doc(),
            style: editor.style,
            alignment: editor.alignment,
            block: editor.block.clone(),
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

impl<'a> Widget for EditorRenderer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut inner = self
            .doc
            .paragraph()
            .style(self.style)
            .alignment(self.alignment);

        if let Some(b) = self.block {
            inner = inner.block(b);
        }

        inner.render(area, buf);
    }
}
