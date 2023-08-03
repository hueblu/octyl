use std::any::Any;

use crate::compositor::CharBuffer;
use crate::error::Result;
use crate::geometry::Rect;

pub trait Component: Any {
    fn render(&mut self, size: (usize, usize)) -> Result<CharBuffer>;
    fn resize(&mut self, size: (usize, usize)) -> Result<()>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
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

impl Component for EmptyComponent {
    fn render(&mut self, size: (usize, usize)) -> Result<CharBuffer> {
        Ok(CharBuffer::new(size.0, size.1))
    }
    fn resize(&mut self, _size: (usize, usize)) -> Result<()> {
        Ok(())
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
