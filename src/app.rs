use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{mpsc, Mutex};

use crate::{
    action::AppAction,
    components::{root::Root, Component},
    config::Config,
    event::EventHandler,
    terminal::TerminalHandler,
    trace_dbg,
};

pub struct App {
    pub config: Config,
    // (tick_rate, render_tick_rate)
    pub tick_rate: (u64, u64),
    pub root: Arc<Mutex<Root>>,
    pub should_quit: bool,
    pub should_suspend: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::new()?;
        let root = Arc::new(Mutex::new(Root::new()));
        Ok(Self {
            tick_rate: (30, 60),
            root,
            should_quit: false,
            should_suspend: false,
            config,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) =
            mpsc::unbounded_channel();

        self.root.lock().await.init(action_tx.clone())?;

        let terminal =
            TerminalHandler::new(self.root.clone());
        let mut event = EventHandler::new(
            self.tick_rate,
            self.root.clone(),
            action_tx.clone(),
        );

        loop {
            if let Some(action) = action_rx.recv().await {
                if let Some(action) = action
                    .as_any()
                    .downcast_ref::<AppAction>(
                ) {
                    if *action != AppAction::Tick
                        && *action != AppAction::RenderTick
                    {
                        trace_dbg!(action);
                    }

                    match action {
                        AppAction::RenderTick => {
                            terminal.render()?
                        },
                        AppAction::Quit => {
                            self.should_quit = true
                        },
                        _ => {},
                    }
                }
                if let Some(_action) = self
                    .root
                    .lock()
                    .await
                    .dispatch(action)
                    .await
                {
                    action_tx.send(_action)?;
                };
            }
            if self.should_quit {
                terminal.stop()?;
                event.stop();
                terminal.task.await?;
                event.task.await?;
                break;
            }
        }
        Ok(())
    }
}
