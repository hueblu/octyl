pub mod identifier;

use std::{any::Any, collections::HashMap};

use identifier::ComponentId;

use crate::{
    app::{BoxMessage, Command},
    tui::Frame,
};

pub type ComponentType = dyn Component<State = Box<dyn Any>>;

pub struct Components {
    components: HashMap<ComponentId, Box<ComponentType>>,
}

#[async_trait::async_trait]
pub trait Component {
    type State: Sized;

    fn init(&self) -> Self::State;
    async fn view<'a>(&self, model: &Self::State, frame: Frame<'a>);
    async fn update(&self, model: &mut Self::State, message: BoxMessage) -> Command;
}

impl Components {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
}
