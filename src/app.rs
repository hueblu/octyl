use std::{
    io::{self},
    panic,
};

use anyhow::Result;
use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{prelude::CrosstermBackend, Frame, Terminal};

use crate::{
    editor::Editor,
    event::{Event, EventHandler},
};

/// The main app struct
pub struct App<'a> {
    // whether the app should close
    // on the next loop
    pub close: bool,

    events: EventHandler,
    editor: Editor<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self> {
        log::info!("Initializing app");

        Ok(Self {
            close: false,
            events: EventHandler::new()?,
            editor: Editor::from("Hello World\n\nPress <esc> to exit".lines()),
        })
    }

    pub async fn run(&mut self) -> Result<i32> {
        log::info!("Running app");

        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

        setup_terminal()?;
        terminal.hide_cursor()?;
        terminal.clear()?;

        while !self.close {
            match self.events.receiver.recv().await? {
                Event::CtrlC => self.close = true,
                Event::Tick => self.tick().await,
                Event::Terminal(event) => self.handle_terminal_event(event).await,
            }

            terminal.draw(|f| {
                self.draw(f).unwrap();
            })?;
        }

        terminal.show_cursor()?;
        cleanup_terminal()?;

        Ok(0)
    }

    pub async fn tick(&mut self) {}

    pub async fn handle_terminal_event(&mut self, event: crossterm::event::Event) {
        match event {
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => self.close = true,

            _ => {}
        }
    }

    pub fn draw(&mut self, frame: &mut Frame<'_, CrosstermBackend<io::Stdout>>) -> Result<()> {
        let area = frame.size();

        frame.render_widget(self.editor.clone(), area);

        Ok(())
    }
}

fn setup_terminal() -> Result<()> {
    log::trace!("Setting up terminal");

    enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        cleanup_terminal().expect("Failed to cleanup terminal");
        panic_hook(panic);
    }));

    Ok(())
}

fn cleanup_terminal() -> Result<()> {
    log::trace!("Cleaning up terminal");

    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
