use anyhow::Result;
use async_trait::async_trait;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::Level;

use super::Component;
use crate::{
    action::{Action, AppAction},
    components::{Layer, LayerType},
    terminal::Frame,
};

#[derive(Default)]
pub struct Root {
    layers: Vec<Layer>,
    action_tx: Option<mpsc::UnboundedSender<Box<dyn Action>>>,
}

impl Root {
    pub fn with_component(mut self, component: Box<dyn Component>) -> Result<Self> {
        let mut layer = Layer::new_tiled(Some(component));
        if let Some(ref tx) = self.action_tx {
            layer.init(tx.clone())?;
        }
        self.layers.push(layer);

        Ok(self)
    }
}

#[async_trait]
impl Component for Root {
    fn init(&mut self, tx: UnboundedSender<Box<dyn Action>>) -> anyhow::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Box<dyn Action> {
        tracing::event!(Level::DEBUG, ?key.code, "Root received key event");

        for layer in &mut self.layers {
            if layer.key_event_opaque() {
                return layer.handle_key_event(key);
            } else {
                layer.handle_key_event(key);
            }
        }
        Box::new(AppAction::Noop)
    }

    fn render(&mut self, f: &mut Frame<'_>, rect: Rect) {
        for layer in &mut self.layers {
            layer.render(f, rect);

            if let LayerType::Tiled { .. } = layer.inner {
                return;
            }
        }
    }

    fn key_event_opaque(&self) -> bool {
        true
    }

    fn mouse_event_opaque(&self) -> bool {
        true
    }
}
