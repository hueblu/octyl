use std::any::Any;

use anyhow::Result;
use ratatui::prelude::{Constraint, Direction};

use crate::component::{Component, ComponentState};

pub struct View {
    component_tree: ComponentTreeNode,
}

pub enum ComponentTreeNode {
    Leaf {
        component: ComponentState,
        constraint: Option<Constraint>,
    },
    Branch {
        children: Vec<Self>,
        direction: Direction,
        focused: Option<usize>,
    },
}

impl View {
    pub fn new() -> Result<Self> {
        Ok(Self {
            component_tree: ComponentTreeNode::root(),
        })
    }

    pub fn add_mounted_component(&mut self, mounted: ComponentState) {
        if let ComponentTreeNode::Branch {
            ref mut children, ..
        } = &mut self.component_tree
        {
            children.push(mounted.into());
        }
    }

    // gets the focused ComponentState, returning None
    // if there is no focused ComponentState
    pub fn get_focused(&mut self) -> Option<&mut ComponentState> {
        self.component_tree.get_focused()
    }
}

impl ComponentTreeNode {
    pub fn root() -> Self {
        Self::Branch {
            children: Vec::new(),
            direction: Direction::Horizontal,
            focused: None,
        }
    }

    pub fn get_focused(&mut self) -> Option<&mut ComponentState> {
        match self {
            Self::Leaf {
                ref mut component, ..
            } => Some(component),
            Self::Branch {
                children, focused, ..
            } => {
                if let Some(focused) = focused
                    && let Some(child) = children.get_mut(*focused)
                {
                    child.get_focused()
                } else {
                    None
                }
            }
        }
    }
}

impl From<ComponentState> for ComponentTreeNode {
    fn from(value: ComponentState) -> Self {
        Self::Leaf {
            component: value,
            constraint: None,
        }
    }
}
