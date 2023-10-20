mod identifier;

use std::{any::Any, collections::HashMap};

use async_trait::async_trait;
use identifier::ComponentId;
use ratatui::prelude::Rect;
use tokio::sync::mpsc;

use crate::{
    app::{BoxMessage, Command},
    tui::Frame,
};

pub struct Components {
    components: HashMap<ComponentId, Box<dyn MockComponent>>,

    msg_tx: mpsc::UnboundedSender<Command>,
}

#[async_trait]
pub trait MockComponent {
    fn init(&self, id: ComponentId) -> MountedComponent;
    fn render(&self, frame: &mut Frame, area: Rect);
    async fn handle_message(&self, message: BoxMessage);
}

pub struct MountedComponent {
    state: HashMap<String, Box<dyn Any>>,
    component_id: ComponentId,
}

impl Components {
    pub fn new(msg_tx: mpsc::UnboundedSender<Command>) -> Self {
        Self {
            components: HashMap::new(),
            msg_tx,
        }
    }
}
