#![allow(dead_code)]

mod events;
mod tui;

use events::{Event, EventHandler};

use anyhow::Result;

pub struct App {
    events: EventHandler,
    close: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            events: EventHandler::new(),
            close: false,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while !self.close {
            tokio::select! {
                event = self.events.next() => {
                    match event {
                        Some(Event::Tick) => {
                            tracing::trace!("tick");
                        }

                        Some(Event::Quit) => {
                            tracing::info!("quit event received");
                            self.close = true;

                        }

                        None => {
                            tracing::info!("event handler closed");
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
