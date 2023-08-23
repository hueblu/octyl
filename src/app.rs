use std::{
    io::{self},
    panic,
};

use anyhow::Result;
use crossterm::{
    event::{Event as CrossEvent, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
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

pub trait Component<'a> {
    type Renderer: tui::widgets::Widget;

    fn as_widget(&'a self) -> Self::Renderer;
    fn get_cursor(&self) -> Option<(u16, u16)>;
    fn render(&'a self, frame: &mut Frame<'_, CrosstermBackend<io::Stdout>>, cursor: bool) {
        let widget = self.as_widget();

        frame.render_widget(widget, frame.size());

        if cursor {
            if let Some((x, y)) = self.get_cursor() {
                frame.set_cursor(x, y);
            }
        }
    }
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
        terminal.clear()?;

        while !self.close {
            match self.events.receiver.recv().await? {
                Event::CtrlC => self.close = true,
                Event::Tick => self.tick().await,
                Event::Terminal(event) => self.handle_terminal_event(event).await?,
            }

            terminal.draw(|f| {
                let _ = self.draw(f);
            })?;
        }

        cleanup_terminal()?;

        Ok(0)
    }

    pub async fn tick(&mut self) {}

    pub async fn handle_terminal_event(&mut self, event: CrossEvent) -> Result<()> {
        match event {
            CrossEvent::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => self.close = true,

            CrossEvent::Key(KeyEvent {
                kind: KeyEventKind::Press | KeyEventKind::Repeat,
                ..
            }) => self.editor.handle_terminal_event(event).await?,

            _ => {}
        }

        Ok(())
    }

    pub fn draw(&mut self, frame: &mut Frame<'_, CrosstermBackend<io::Stdout>>) -> Result<()> {
        self.editor.render(frame, true);
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
