use anyhow::Result;
use async_trait::async_trait;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    layout::{Constraint, Direction, Rect},
    prelude::Layout,
};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{
    action::{Action, AppAction},
    event::Event,
    terminal::Frame,
};

pub mod logger;
pub mod root;

type Window = Box<dyn Component>;

#[async_trait]
pub trait Component: Send + Sync {
    #[allow(unused_variables)]
    fn init(&mut self, tx: UnboundedSender<Box<dyn Action>>) -> Result<()> {
        Ok(())
    }

    async fn handle_event(&mut self, event: Option<Event>) -> Box<dyn Action> {
        if let Some(event) = event {
            if let Event::Key(key_event) = event {
                return self.handle_key_event(key_event);
            } else if let Event::Mouse(mouse_event) = event {
                return self.handle_mouse_event(mouse_event);
            };

            Box::new(match event {
                Event::Quit => AppAction::Quit,
                Event::AppTick => AppAction::Tick,
                Event::RenderTick => AppAction::RenderTick,

                _ => AppAction::Noop,
            })
        } else {
            Box::new(AppAction::Noop)
        }
    }

    /// Checks a KeyEvent and returns
    /// an action to dispatch.
    ///
    /// Shouldn't update state or be
    /// called directly.
    #[allow(unused_variables)]
    fn handle_key_event(&mut self, key: KeyEvent) -> Box<dyn Action> {
        Box::new(AppAction::Noop)
    }

    /// Checks a MouseEvent and returns
    /// an action to dispatch.
    ///
    /// Shouldn't update state or be
    /// called directly.
    #[allow(unused_variables)]
    fn handle_mouse_event(&mut self, mouse: MouseEvent) -> Box<dyn Action> {
        Box::new(AppAction::Noop)
    }
    /// Returns true if the component should
    /// consume the key events it receives.
    #[allow(unused_variables)]
    fn key_event_opaque(&self) -> bool {
        false
    }

    /// Returns true if the component should
    /// consume the mouse events it receives.
    #[allow(unused_variables)]
    fn mouse_event_opaque(&self) -> bool {
        false
    }

    /// Consumes an action to update state
    /// and optionally returns an action to
    /// dispatch on another tick.
    #[allow(unused_variables)]
    async fn dispatch(&mut self, action: Box<dyn Action>) -> Option<Box<dyn Action>> {
        None
    }

    /// Renders the component to the given frame.
    fn render(&mut self, f: &mut Frame<'_>, rect: Rect);
}

impl Default for Box<dyn Component> {
    fn default() -> Self {
        Box::new(EmptyComponent)
    }
}

#[derive(Debug)]
pub struct Layer {
    pub inner: LayerType,
    action_tx: Option<mpsc::UnboundedSender<Box<dyn Action>>>,
}

pub enum LayerType {
    Floating { component: Window, rect: Rect },
    Tiled { root_node: ComponentTreeNode },
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

impl Layer {
    /// creates a new tiling layer with the given component
    /// if no component is given, layer will be created with
    /// an empty component
    pub fn new_tiled(component: Option<Box<dyn Component>>) -> Self {
        Self {
            inner: LayerType::new_tiled(component),
            action_tx: None,
        }
    }

    /// creates a new floating layer with the given component
    /// if no component is given, layer will be created with
    /// an empty component
    pub fn new_floating(component: Option<Box<dyn Component>>, rect: Rect) -> Self {
        Self {
            inner: LayerType::new_floating(component, rect),
            action_tx: None,
        }
    }

    pub fn with_component(mut self, c: Box<dyn Component>) -> Self {
        match self.inner {
            LayerType::Tiled { ref mut root_node } => {
                root_node.extend(ComponentTreeNode::new_leaf(c));
            }
            LayerType::Floating { component, .. } => {
                self = Self::new_tiled(Some(component)).with_component(c);
            }
        }
        self
    }

    pub fn add_component(&mut self, c: Box<dyn Component>) {
        match &mut self.inner {
            LayerType::Tiled { root_node } => {
                root_node.extend(ComponentTreeNode::new_leaf(c));
            }
            LayerType::Floating { component, .. } => *component = c,
        }
    }
}

impl Component for Layer {
    fn init(&mut self, tx: UnboundedSender<Box<dyn Action>>) -> Result<()> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Box<dyn Action> {
        match &mut self.inner {
            LayerType::Tiled { ref mut root_node } => root_node
                .get_focused_mut()
                .map(|c| c.handle_key_event(key))
                .unwrap_or(Box::new(AppAction::Noop)),
            LayerType::Floating {
                ref mut component, ..
            } => component.handle_key_event(key),
        }
    }

    fn key_event_opaque(&self) -> bool {
        match &self.inner {
            LayerType::Tiled { root_node } => root_node
                .get_focused()
                .map(|c| c.key_event_opaque())
                .unwrap_or(false),
            LayerType::Floating { component, .. } => component.key_event_opaque(),
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, rect: Rect) {
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
                        .direction(*direction)
                        .constraints(constraints.clone().into_boxed_slice())
                        .split(rect)
                        .to_vec();

                    for (child, rect) in children.iter_mut().zip(rects) {
                        render_node(child, f, rect);
                    }
                }
            }
        }

        match &mut self.inner {
            LayerType::Floating { component, rect } => {
                component.render(f, *rect);
            }

            LayerType::Tiled { root_node } => {
                render_node(root_node, f, rect);
            }
        }
    }
}

impl LayerType {
    pub fn new_tiled(component: Option<Box<dyn Component>>) -> Self {
        Self::Tiled {
            root_node: ComponentTreeNode::new_leaf(component.unwrap_or(Box::new(EmptyComponent))),
        }
    }

    pub fn new_floating(component: Option<Box<dyn Component>>, rect: Rect) -> Self {
        Self::Floating {
            component: component.unwrap_or_default(),
            rect,
        }
    }
}

impl std::fmt::Debug for LayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayerType::Floating { .. } => write!(f, "Floating"),
            LayerType::Tiled { root_node } => write!(f, "Tiled: {:?}", root_node),
        }
    }
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

    pub fn new_leaf(component: Box<dyn Component>) -> Self {
        Self::Leaf { component }
    }

    pub fn get_focused(&self) -> Option<&dyn Component> {
        match self {
            Self::Leaf { component } => Some(component.as_ref()),
            Self::Branch {
                children, focused, ..
            } => children.get(*focused).and_then(|node| node.get_focused()),
        }
    }

    pub fn get_focused_mut(&mut self) -> Option<&mut dyn Component> {
        match self {
            Self::Leaf { component } => Some(component.as_mut()),
            Self::Branch {
                children, focused, ..
            } => children
                .get_mut(*focused)
                .and_then(|node| node.get_focused_mut()),
        }
    }

    pub fn get_children(&mut self) -> Option<&Vec<ComponentTreeNode>> {
        match *self {
            Self::Branch { ref children, .. } => Some(children),
            _ => None,
        }
    }

    pub fn extend(&mut self, other: ComponentTreeNode) {
        if let Self::Branch {
            ref mut children, ..
        } = self
        {
            children.push(other)
        }
    }

    pub fn leaf_to_branch(self) -> Self {
        Self::Branch {
            children: vec![self],
            direction: Direction::Horizontal,
            constraints: vec![],
            focused: 0,
        }
    }
}

impl Default for ComponentTreeNode {
    fn default() -> Self {
        Self::Leaf {
            component: Box::new(EmptyComponent),
        }
    }
}

impl std::fmt::Debug for ComponentTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentTreeNode::Leaf { .. } => write!(f, "Leaf"),
            ComponentTreeNode::Branch {
                children,
                direction,
                constraints,
                focused,
            } => write!(
                f,
                "Branch: {:?} children,  {:?}, {:?}, {:?} focused",
                children.len(),
                direction,
                constraints,
                focused
            ),
        }
    }
}

pub struct EmptyComponent;
impl EmptyComponent {
    pub(crate) fn default() -> logger::Logger {
        todo!()
    }
}

impl Component for EmptyComponent {
    fn render(&mut self, _f: &mut Frame<'_>, _rect: Rect) {}
}
