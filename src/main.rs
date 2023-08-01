#![allow(dead_code)]
use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use octyl::{
    compositor::{Component, Compositor},
    document::Document,
    error::Result,
};

fn main() -> Result<()> {
    init_terminal()?;

    let mut close = false;

    let screen_size = crossterm::terminal::size()?;
    let mut doc = Document::new((screen_size.0 as usize, screen_size.1 as usize));

    let mut compositor = Compositor::new();
    let (mut width, mut height) = crossterm::terminal::size()?;
    let mut cursor_pos: (u16, u16);

    while !close {
        let new_cursor_coords = doc.cursor_coords();
        cursor_pos = (new_cursor_coords.0 as u16, new_cursor_coords.1 as u16);

        execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1))?;

        compositor.draw(|frame| {
            frame.render_component(&mut doc);
        });

        match read()? {
            Event::Key(keyevent) => match keyevent.code {
                KeyCode::Esc => close = true,
                KeyCode::Char(c) => {
                    doc.insert_char(c);

                    cursor_pos.0 += 1;
                    if cursor_pos.0 > width {
                        cursor_pos.0 = 0;
                        cursor_pos.1 += 1;
                    }
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                (width, height) = (w, h);
            }
            _ => {}
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    }

    cleanup_terminal()?;
    Ok(())
}

fn init_terminal() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All))?;
    Ok(())
}

fn cleanup_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    Ok(())
}
