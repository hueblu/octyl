use crate::{
    action::Action,
    component::ComponentCollection,
    components,
    event::{Event, EventProducer, EventProducerCollection, EventTrait},
    view::View,
};
use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use tracing::error;

pub struct App {
    pub should_close: bool,

    pub view: View,
    pub components: ComponentCollection,

    pub event_producers: EventProducerCollection,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut stderr = std::io::stderr();

        stderr.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        Ok(App {
            should_close: false,

            view: View::new()?,
            components: ComponentCollection::new(),

            event_producers: EventProducerCollection::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let logger_id = self
            .components
            .register_component(components::logger::logger_component())?;
        self.view
            .add_mounted_component(self.components.new_mounted(logger_id)?);

        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        let action_stream = UnboundedReceiverStream::new(action_rx);

        loop {
            if let Some(event) = self.event_producers.next().await {
                self.handle_event(event, &action_tx).await?;
            };

            if self.should_close {
                break;
            }
        }

        Ok(())
    }

    pub async fn handle_event(
        &mut self,
        event: Event,
        action_tx: &mpsc::UnboundedSender<Action>,
    ) -> Result<()> {
        if let Some(state) = self.view.get_focused() {
            self.components
                .dispatch_event(event, &state.component_id, action_tx)?;
        };

        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        Ok(())
    }

    pub async fn draw(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let mut stderr = std::io::stderr();
        let _ = stderr.execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
