use crossterm::event::{
    Event as CrossEvent, KeyCode as CrossKeyCode, KeyEvent as CrossKeyEvent,
    KeyEventKind as CrossKeyEventKind, KeyModifiers as CrossKeyModifiers,
};

use crate::geometry::Position;

#[derive(Debug, Clone)]
pub enum TerminalEvent {
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    Paste(String),
    Resize(Position),
    Unsupported,
}

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    pub kind: KeyEventKind,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    Unsupported,
}

#[derive(Debug, Clone)]
pub struct KeyModifiers {
    pub shift: bool,
    pub alt: bool,
    pub control: bool,
}

#[derive(Debug, Clone)]
pub enum KeyEventKind {
    Press,
    Release,
    Repeat,
}

impl From<CrossEvent> for TerminalEvent {
    fn from(value: CrossEvent) -> Self {
        match value {
            CrossEvent::FocusGained => Self::FocusGained,
            CrossEvent::FocusLost => Self::FocusLost,
            CrossEvent::Key(keyevent) => Self::Key(keyevent.into()),
            CrossEvent::Paste(paste) => Self::Paste(paste),
            CrossEvent::Resize(x, y) => Self::Resize(Position::from((x, y))),
            _ => Self::Unsupported,
        }
    }
}

impl From<CrossKeyEvent> for KeyEvent {
    fn from(value: CrossKeyEvent) -> Self {
        Self {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
            kind: value.kind.into(),
        }
    }
}

impl From<CrossKeyCode> for KeyCode {
    fn from(value: CrossKeyCode) -> Self {
        match value {
            CrossKeyCode::Backspace => Self::Backspace,
            CrossKeyCode::Enter => Self::Enter,
            CrossKeyCode::Left => Self::Left,
            CrossKeyCode::Right => Self::Right,
            CrossKeyCode::Up => Self::Up,
            CrossKeyCode::Down => Self::Down,
            CrossKeyCode::Home => Self::Home,
            CrossKeyCode::End => Self::End,
            CrossKeyCode::PageUp => Self::PageUp,
            CrossKeyCode::PageDown => Self::PageDown,
            CrossKeyCode::Tab => Self::Tab,
            CrossKeyCode::BackTab => Self::BackTab,
            CrossKeyCode::Delete => Self::Delete,
            CrossKeyCode::Insert => Self::Insert,
            CrossKeyCode::F(n) => Self::F(n),
            CrossKeyCode::Char(c) => Self::Char(c),
            CrossKeyCode::Null => Self::Null,
            CrossKeyCode::Esc => Self::Esc,
            CrossKeyCode::CapsLock => Self::CapsLock,
            CrossKeyCode::ScrollLock => Self::ScrollLock,
            CrossKeyCode::NumLock => Self::NumLock,
            CrossKeyCode::PrintScreen => Self::PrintScreen,
            CrossKeyCode::Pause => Self::Pause,
            CrossKeyCode::Menu => Self::Menu,
            CrossKeyCode::KeypadBegin => Self::KeypadBegin,
            _ => Self::Unsupported,
        }
    }
}

impl From<CrossKeyModifiers> for KeyModifiers {
    fn from(value: CrossKeyModifiers) -> Self {
        Self {
            shift: value.contains(CrossKeyModifiers::SHIFT),
            alt: value.contains(CrossKeyModifiers::ALT),
            control: value.contains(CrossKeyModifiers::CONTROL),
        }
    }
}

impl From<CrossKeyEventKind> for KeyEventKind {
    fn from(value: CrossKeyEventKind) -> Self {
        match value {
            CrossKeyEventKind::Press => Self::Press,
            CrossKeyEventKind::Release => Self::Release,
            CrossKeyEventKind::Repeat => Self::Repeat,
        }
    }
}
