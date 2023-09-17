use async_trait::async_trait;
use crossterm::event::KeyEvent;
use ratatui::layout::{Layout, Rect};
use tokio::sync::mpsc::{self, UnboundedSender};
use tracing::Level;

use super::{Component, ComponentTreeNode};
use crate::{
    action::{Action, AppAction},
    components::Layer,
    terminal::Frame,
};

#[derive(Default)]
pub struct Root {
    layers: Vec<Layer>,
    action_tx: Option<mpsc::UnboundedSender<Box<dyn Action>>>,
}

impl Root {
    pub fn with_component(mut self, component: Box<dyn Component>) -> Self {
        self.layers.push(Layer::Tiled {
            root_node: ComponentTreeNode::Leaf { component },
            active: 0,
        });
        self
    }
}

#[async_trait]
impl Component for Root {
    fn init(&mut self, tx: UnboundedSender<Box<dyn Action>>) -> anyhow::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Box<dyn Action> {
        tracing::event!(Level::DEBUG, ?key.code, "Root received key event");

        Box::new(AppAction::Noop)
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        fn render_node(root: &mut ComponentTreeNode, f: &mut Frame<'_>, rect: Rect) {
            match root {
                ComponentTreeNode::Leaf { component } => component.render(f, rect),
                ComponentTreeNode::Branch {
                    children,
                    direction,
                    constraints,
                    ..
                } => {
                    if children.is_empty() {
                        return;
                    };

                    let rects = Layout::default()
                        .direction(direction.clone())
                        .constraints(constraints.clone().into_boxed_slice())
                        .split(rect)
                        .to_vec();

                    for (child, rect) in children.iter_mut().zip(rects) {
                        render_node(child, f, rect);
                    }
                }
            }
        }

        for layer in &mut self.layers {
            match layer {
                Layer::Floating {
                    ref mut component,
                    rect,
                } => {
                    component.render(f, *rect);
                }

                Layer::Tiled { root_node, .. } => {
                    render_node(root_node, f, area);
                }
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
