use super::component::Component;
use super::component::Window;
use crate::geometry::Split;
use crate::CharBuffer;

// given to user to manipulate
// during drawing closure
pub struct Frame {
    layers: Vec<Layer>,
}

pub enum Layer {
    Tiled(TiledWindowTree),
    Floating(Window),
    Empty,
}

enum TiledWindowTree {
    Leaf(Option<Box<dyn Component>>),
    Branch {
        windows: (Box<TiledWindowTree>, Box<TiledWindowTree>),
        split: Split,
    },
}

impl Frame {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// add a layer to the front of the frame
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn render(self) -> CharBuffer {
        todo!()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl Layer {
    pub fn new_tiled() -> Self {
        Self::Tiled(TiledWindowTree::Leaf(None))
    }
}
