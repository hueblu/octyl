use tokio::sync::mpsc;

use crate::{action::Action, components::Component};

pub struct Editor {
    action_tx: mpsc::UnboundedSender<Box<dyn Action>>,
}

impl Component for Editor {
    fn init(
        &mut self,
        tx: mpsc::UnboundedSender<Box<dyn crate::action::Action>>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn render(&mut self, f: &mut crate::terminal::Frame<'_>, rect: ratatui::prelude::Rect) {}
}
