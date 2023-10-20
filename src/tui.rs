use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    event::{DisableBracketedPaste, DisableMouseCapture, KeyEvent, MouseEvent},
    terminal::LeaveAlternateScreen,
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend as Backend, Terminal};
use tokio::{sync::mpsc::UnboundedReceiver, task::JoinHandle};
use tokio_util::sync::CancellationToken;

pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stderr>>;

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct Tui {
    pub terminal: Terminal<Backend<std::io::Stderr>>,
    task: JoinHandle<()>,
    cancellation_token: CancellationToken,
    pub event_rx: UnboundedReceiver<Event>,
    render_rate: f64,
    tick_rate: f64,
}

impl Tui {
    pub fn init() -> Self {
        // enter raw mode
        // enter alternate screen
        // enable bracketed Paste
        // enable mouse capture
    }

    pub fn exit(&self) -> Result<()> {
        self.cancellation_token.cancel();
        let mut counter = 0;
        while !self.task.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
            counter += 1;
            if counter > 50 {
                self.task.abort();
            }

            if counter > 100 {
                log::error!("Failed to abort task in 100 milliseconds for unknown reason")
            }
        }

        std::io::stderr()
            .execute(DisableBracketedPaste)?
            .execute(DisableMouseCapture)?
            .execute(LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }
}

impl Deref for Tui {
    type Target = Terminal<Backend<std::io::Stderr>>;

    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Tui {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}
