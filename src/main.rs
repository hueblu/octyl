#![allow(dead_code)]

use std::{io::stdout, sync::OnceLock};

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType},
};

use octyl::{
    compositor::{
        component::AsComponent,
        frame::{Layer, TiledWindowTree},
        Compositor,
    },
    document::Document,
    error::Result,
    terminal::Terminal,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = Terminal::init()?;

    let mut close = false;

    let screen_size = terminal.size();
    let mut doc = Document::new(screen_size);

    let mut compositor = Compositor::new(screen_size);
    let (mut width, mut _height) = crossterm::terminal::size()?;
    let mut cursor_pos: (u16, u16);

    while !close {
        let new_cursor_coords = doc.cursor_coords();
        cursor_pos = (new_cursor_coords.0 as u16, new_cursor_coords.1 as u16);

        execute!(stdout(), MoveTo(cursor_pos.0, cursor_pos.1))?;

        compositor.draw(|frame| -> Result<()> {
            frame.add_layer(Layer::Tiled(TiledWindowTree::Leaf(Some(
                doc.as_component(),
            ))));

            Ok(())
        })?;

        compositor.render()?;

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
                (width, _height) = (w, h);
            }
            _ => {}
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    }

    Ok(())
}
