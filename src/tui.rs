use ratatui::backend::CrosstermBackend as Backend;

pub type Frame<'a> = ratatui::Frame<'a, Backend<std::io::Stderr>>;

pub struct Tui {}

impl Tui {
    pub fn new() {}
}
