#![allow(dead_code)]

mod events;
mod term;

use events::EventHandler;

use anyhow::Result;

pub struct App {
    events: EventHandler,
}

impl App {
    pub fn new() -> Self {
        App {
            events: EventHandler::new(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
