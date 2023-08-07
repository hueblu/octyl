pub mod event;

use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    event::EventStream,
    execute, queue,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use futures::StreamExt;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio_util::sync::CancellationToken;

use crate::{geometry::Position, CharBuffer, Result};

use self::event::TerminalEvent;

pub struct Terminal {
    out: Stdout,
    size: Position,
    focused: bool,
    cursor_coords: Position,
    event_queue: Receiver<TerminalEvent>,
}

impl Terminal {
    /// Initializes the terminal
    pub fn init() -> Result<Self> {
        let mut out = stdout();
        let size = crossterm::terminal::size().unwrap();
        let size = Position::from(size);
        let focused = true;
        let (tx, rx) = broadcast::channel(25);

        Self::spawn_event_thread(tx)?;
        enable_raw_mode()?;
        execute!(out, EnterAlternateScreen, Clear(ClearType::All))?;

        Ok(Self {
            out,
            size,
            focused,
            cursor_coords: Position { x: 0, y: 0 },
            event_queue: rx,
        })
    }

    fn spawn_event_thread(sender: Sender<TerminalEvent>) -> Result<CancellationToken> {
        let token = CancellationToken::new();
        let token_clone = token.clone();
        let mut event_stream = EventStream::new();

        tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    _ = token_clone.cancelled() => break,
                    event = event_stream.next() => {
                        if let Some(event) = event {
                            match event {
                                Ok(event) => {
                                    //TODO: actual error handling
                                    if let Err(_) = sender.send(TerminalEvent::from(event)) {
                                        break;
                                    }
                                },
                                Err(_) => break,
                            }
                        }
                    }
                }
            }
        });

        Ok(token)
    }

    /// Cleans up the terminal before exiting
    /// Terminal should be dropped immediately after
    /// calling this function
    pub fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.out,
            LeaveAlternateScreen,
            Clear(ClearType::All),
            MoveTo(0, 0)
        )?;

        Ok(())
    }

    pub fn move_cursor_to(&mut self, x: u16, y: u16) -> Result<()> {
        queue!(self.out, MoveTo(x, y))?;
        Ok(())
    }

    // write the contents of a char buffer to the screen
    //
    pub fn write_char_buffer(&mut self, buf: CharBuffer) -> Result<()> {
        queue!(self.out, MoveTo(0, 0))?;
        for (y, row) in buf.rows().iter().enumerate() {
            queue!(self.out, MoveTo(0, y as u16))?;

            self.out.write_all(row.as_bytes())?;
        }

        Ok(())
    }

    pub fn size(&self) -> Position {
        self.size
    }
}

impl Write for Terminal {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.out.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.out.write_all(buf)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}
