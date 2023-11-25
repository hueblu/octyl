use crossterm::event::{KeyEvent, MouseEvent};

use crate::event::EventTrait;

const TICK_RATE: f64 = 1. / 60.;

#[derive(Clone, PartialEq, Debug)]
pub enum TerminalEvent {
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Error,
}

pub enum AppEvent {
    Tick,
    Init,
    Quit,
    Error,
}
