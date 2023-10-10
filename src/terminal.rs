use crossterm::event::{Event, EventStream};
use futures::StreamExt;

use crate::app::{App, BoxMessage, BoxMessageStream};

#[derive(PartialEq, Debug, Clone)]
pub struct TerminalEvent(Event);

pub fn terminal_subscription(app: App) -> (App, BoxMessageStream) {
    (
        app,
        Box::pin(EventStream::new().map(|event| {
            if let Ok(event) = event {
                Box::new(TerminalEvent(event)) as BoxMessage
            } else {
                panic!("event stream error")
            }
        })),
    )
}
