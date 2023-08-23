/*
 * stolen DIRECTLY from https://github.com/rhysd/tui-textarea/
 */

use tui::{
    prelude::Rect,
    text::{Line, Text},
    widgets::{Paragraph, StatefulWidget, Widget},
};

#[derive(Clone)]
pub struct TextBuffer {
    lines: Vec<String>,
}

impl<I> From<I> for TextBuffer
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    fn from(value: I) -> Self {
        Self::new(value.into_iter().map(|s| s.into()).collect())
    }
}

impl TextBuffer {
    pub fn new(mut lines: Vec<String>) -> Self {
        if lines.is_empty() {
            lines.push(String::new());
        }

        Self { lines }
    }

    pub fn text<'a>(&self) -> Text<'a> {
        self.lines
            .iter()
            .map(|s| s.clone().into())
            .collect::<Vec<Line>>()
            .into()
    }

    pub fn insert_char<C: Into<char>>(&mut self, coords: (u16, u16), c: C) {
        self.lines[coords.1 as usize].insert(coords.0 as usize, c.into());
    }

    pub fn delete_char(&mut self, coords: (u16, u16)) {
        let line = &mut self.lines[coords.1 as usize];

        if !(coords.0 as usize > line.len()) {
            line.remove(coords.0 as usize);
        }
    }

    pub fn insert_newline(&mut self, coords: (u16, u16)) {
        let newline = self.lines[coords.1 as usize].split_off(coords.0 as usize);
        self.lines.insert(coords.1 as usize + 1, newline);
    }
}

pub struct TextBufferState {
    pub scroll: usize,
    pub wrap: bool,
}

impl StatefulWidget for TextBuffer {
    type State = TextBufferState;

    fn render(self, area: Rect, buf: &mut tui::prelude::Buffer, state: &mut Self::State) {
        //TODO: text wrap
        let paragraph = Paragraph::new(self.text()).scroll((0, state.scroll as u16));
        paragraph.render(area, buf);
    }
}

impl Default for TextBufferState {
    fn default() -> Self {
        Self {
            scroll: 0,
            wrap: false,
        }
    }
}
