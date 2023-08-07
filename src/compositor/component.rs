use std::any::Any;

use crate::compositor::CharBuffer;
use crate::error::Result;
use crate::geometry::{Position, Rect};

pub trait Component: Any {
    fn render(&mut self, size: Position) -> Result<CharBuffer>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

pub trait AsComponent {
    fn as_component(&self) -> Box<dyn Component>;
}

pub struct Window {
    component: Box<dyn Component>,
    rect: Rect,
}

impl Window {
    pub fn new(rect: Rect) -> Self {
        Self {
            component: Box::new(EmptyComponent::default()),
            rect,
        }
    }
}

#[derive(Default)]
pub struct EmptyComponent;

#[derive(Default)]
pub struct TextComponent {
    pub text: String,
}

impl Component for EmptyComponent {
    fn render(&mut self, size: Position) -> Result<CharBuffer> {
        Ok(CharBuffer::new(size))
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl Component for TextComponent {
    fn render(&mut self, size: Position) -> Result<CharBuffer> {
        Ok(CharBuffer::new(size).with_data(self.text.chars().collect()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
