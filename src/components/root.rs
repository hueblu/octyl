use async_trait::async_trait;
use ratatui::layout::{
    Alignment, Constraint, Direction, Rect,
};
use tokio::sync::mpsc::{self, UnboundedSender};

use super::Component;
use crate::{action::Action, terminal::Frame};

type Window = Box<dyn Component>;

pub enum Layer {
    Floating {
        component: Window,
        rect: Rect,
    },
    Tiled {
        root_node: ComponentTreeNode,
        alignment: Alignment,
        active: usize,
    },
}

pub enum ComponentTreeNode {
    Leaf {
        component: Window,
    },
    Branch {
        children: Vec<ComponentTreeNode>,
        direction: Direction,
        constraints: Vec<Constraint>,
        focused: usize,
    },
}

#[derive(Default)]
pub struct Root {
    layers: Vec<Box<Layer>>,
    action_tx:
        Option<mpsc::UnboundedSender<Box<dyn Action>>>,
}

impl ComponentTreeNode {
    pub fn new() -> Self {
        Self::Branch {
            children: vec![],
            direction: Direction::Horizontal,
            constraints: vec![],
            focused: 0,
        }
    }
}

impl Root {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Component for Root {
    fn init(
        &mut self,
        tx: UnboundedSender<Box<dyn Action>>,
    ) -> anyhow::Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    async fn render(
        &mut self,
        f: &mut Frame<'_>,
        area: Rect,
    ) {
        fn render_node(
            root: &mut ComponentTreeNode,
            area: Rect,
        ) {
            match root {
                ComponentTreeNode::Leaf { component } => {},
                ComponentTreeNode::Branch {
                    children,
                    direction,
                    constraints,
                    focused,
                } => {
                    if children.is_empty() {
                        return;
                    }
                },
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
