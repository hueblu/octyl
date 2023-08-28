use std::{any::Any, io, ops::Deref};

use tui::{
    prelude::{CrosstermBackend, Direction, Rect},
    widgets::Widget,
    Frame,
};

use crate::editor::Editor;

pub type BoxedComponent<'a> = Box<dyn Component<'a, Renderer = dyn tui::widgets::Widget + 'a>>;

pub trait Component<'a> {
    type Renderer
    where
        : Deref<Target = dyn Widget>;

    fn as_widget(&'a self) -> Self::Renderer;
    fn get_cursor(&self) -> Option<(u16, u16)>;
    fn render(&'a self, frame: &mut Frame<'_, CrosstermBackend<io::Stdout>>, cursor: bool) {
        let widget = self.as_widget();

        frame.render_widget(widget, frame.size());

        if cursor {
            if let Some((x, y)) = self.get_cursor() {
                frame.set_cursor(x, y);
            }
        }
    }
}

pub struct Layers<'a> {
    pub layers: Vec<Layer<'a>>,
}

pub enum Layer<'a> {
    Floating(BoxedComponent<'a>, Rect),
    Tiled(LayerNode),
}

pub enum LayerNode {
    Leaf(BoxedComponent<'static>),
    Branch {
        nodes: Vec<LayerNode>,
        direction: Direction,
    },
}

impl<'a> Layers<'a> {
    pub fn new() -> Self {
        let boxed_editor: BoxedComponent<'_> = Box::new(Editor::new());

        Self {
            layers: vec![boxed_editor.into()],
        }
    }
}

pub struct LayersRenderer<'a> {
    pub layers: &'a Layers<'a>,
}

impl<'a> Widget for LayersRenderer<'a> {
    fn render(self, area: Rect, buf: &mut tui::prelude::Buffer) {}
}

impl<'a> Component<'a> for Layers<'a> {
    type Renderer = LayersRenderer<'a>;

    fn as_widget(&'a self) -> Self::Renderer {
        Self::Renderer {}
    }

    fn get_cursor(&self) -> Option<(u16, u16)> {
        self.layers[0].get_cursor()
    }
}

impl<'a> From<BoxedComponent<'a>> for Layer<'a> {
    fn from(value: BoxedComponent<'a>) -> Self {
        Self::Tiled(LayerNode::Leaf(value))
    }
}
