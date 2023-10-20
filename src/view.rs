use ratatui::prelude::Rect;

use crate::component::MountedComponent;

#[derive(Default)]
pub struct View {
    layers: Vec<Layer>,
}

pub struct Layer {
    layer_type: LayerType,
    transparent: bool,
}

pub enum LayerType {
    Floating {
        area: Rect,
        component: MountedComponent,
    },
    Tiled(ComponentTree),
}

pub enum ComponentTree {
    Branch { children: Vec<ComponentTree> },
    Leaf { component: MountedComponent },
}

impl View {
    pub fn new() -> Self {
        Self::default()
    }
}
