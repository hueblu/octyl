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

use octyl::{compositor::ScreenBuffer, error::Result};

fn main() -> Result<()> {
    init_terminal()?;

    let mut close = false;
    let mut buff = ScreenBuffer::new();
    let (mut width, mut height) = crossterm::terminal::size()?;
    let mut cursor_pos = (0, 0);

    while !close {
        execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1))?;
        match read()? {
            Event::Key(keyevent) => match keyevent.code {
                KeyCode::Esc => close = true,
                KeyCode::Char(c) => {
                    buff.write_char_at_cursor(c)?;

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
        buff.render()?;
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
    execute!(stdout(), LeaveAlternateScreen, Clear(ClearType::All))?;
    Ok(())
}
