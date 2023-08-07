use super::component::Component;
use super::component::Window;
use crate::geometry::Position;
use crate::geometry::Split;
use crate::CharBuffer;

// given to user to manipulate
// during drawing closure
pub struct Frame {
    layers: Vec<Layer>,
    screen_size: Position,
}

pub enum Layer {
    Tiled(TiledWindowTree),
    Floating(Window),
    Empty,
}

pub enum TiledWindowTree {
    Leaf(Option<Box<dyn Component>>),
    Branch {
        windows: (Box<TiledWindowTree>, Box<TiledWindowTree>),
        split: Split,
    },
}

impl Frame {
    pub fn new(screen_size: Position) -> Self {
        Self {
            layers: Vec::new(),
            screen_size,
        }
    }

    /// add a layer to the front of the frame
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn render(&self) -> CharBuffer {
        CharBuffer::new(self.screen_size)
    }
}

impl Layer {
    pub fn new_tiled(component: Option<Box<dyn Component>>) -> Self {
        Self::Tiled(TiledWindowTree::Leaf(component))
    }
}
