use ratatui::prelude::Rect;

use crate::component::{identifier::ComponentId, Component, ComponentType};

pub struct MountedComponent<C>
where
    C: Component + Sized,
{
    state: C::State,
    id: ComponentId,
}

pub struct View {
    layers: Vec<Layer>,
}

pub enum Layer {
    Floating(Rect, MountedComponent<ComponentType>),
}

impl Default for View {
    fn default() -> Self {
        Self { layers: Vec::new() }
    }
}

impl View {
    pub fn new() -> Self {
        Self::default()
    }
}
