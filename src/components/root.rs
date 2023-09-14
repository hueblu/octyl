use async_trait::async_trait;
use ratatui::layout::{
    Constraint, Direction, Layout, Rect,
};
use tokio::sync::mpsc::{self, UnboundedSender};

use super::{logger::Logger, Component};
use crate::{action::Action, terminal::Frame};

type Window = Box<dyn Component>;

pub enum Layer {
    Floating { component: Window, rect: Rect },
    Tiled { root_node: ComponentTreeNode, active: usize },
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
    layers: Vec<Layer>,
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
        let mut root = Self::default();

        root.layers.push(Layer::Tiled {
            root_node: ComponentTreeNode::Leaf {
                component: Box::new(Logger::default()),
            },
            active: 0,
        });

        root
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

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        fn render_node<'a, 'b>(
            root: &mut ComponentTreeNode,
            f: &'a mut Frame<'b>,
            rect: Rect,
        ) {
            match root {
                ComponentTreeNode::Leaf { component } => {
                    component.render(f, rect)
                },
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
                        .constraints(
                            constraints
                                .clone()
                                .into_boxed_slice(),
                        )
                        .split(rect)
                        .to_vec();

                    for (child, rect) in
                        children.into_iter().zip(rects)
                    {
                        render_node(child, f, rect);
                    }
                },
            }
        }

        for layer in &mut self.layers {
            match layer {
                Layer::Floating {
                    ref mut component,
                    rect,
                } => {
                    component.render(f, *rect);
                },

                Layer::Tiled { root_node, .. } => {
                    render_node(root_node, f, area);
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
