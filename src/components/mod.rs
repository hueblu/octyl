use anyhow::Result;
use async_trait::async_trait;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::{Action, AppAction},
    event::Event,
    terminal::Frame,
};

pub mod logger;
pub mod root;

#[async_trait]
pub trait Component: Send + Sync {
    #[allow(unused_variables)]
    fn init(
        &mut self,
        tx: UnboundedSender<Box<dyn Action>>,
    ) -> Result<()> {
        Ok(())
    }

    async fn handle_events(
        &mut self,
        event: Option<Event>,
    ) -> Box<dyn Action> {
        match event {
            Some(Event::Quit) => Box::new(AppAction::Quit),
            Some(Event::AppTick) => {
                Box::new(AppAction::Tick)
            },
            Some(Event::RenderTick) => {
                Box::new(AppAction::RenderTick)
            },
            Some(Event::Key(key_event)) => {
                self.handle_key_events(key_event)
            },
            Some(Event::Mouse(mouse_event)) => {
                self.handle_mouse_events(mouse_event)
            },
            Some(Event::Resize(x, y)) => {
                Box::new(AppAction::Resize(x, y))
            },
            Some(_) => Box::new(AppAction::Noop),
            None => Box::new(AppAction::Noop),
        }
    }

    /// Checks a KeyEvent and returns
    /// an action to dispatch.
    ///
    /// Shouldn't update state or be
    /// called directly.
    #[allow(unused_variables)]
    fn handle_key_events(
        &mut self,
        key: KeyEvent,
    ) -> Box<dyn Action> {
        Box::new(AppAction::Noop)
    }

    /// Checks a MouseEvent and returns
    /// an action to dispatch.
    ///
    /// Shouldn't update state or be
    /// called directly.
    #[allow(unused_variables)]
    fn handle_mouse_events(
        &mut self,
        mouse: MouseEvent,
    ) -> Box<dyn Action> {
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
    async fn dispatch(
        &mut self,
        action: Box<dyn Action>,
    ) -> Option<Box<dyn Action>> {
        None
    }

    /// Renders the component to the given frame.
    fn render(&mut self, f: &mut Frame<'_>, rect: Rect);
}
