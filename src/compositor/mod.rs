pub mod component;
pub mod frame;

use crate::{error::Result, CharBuffer};
pub use component::Component;
pub use frame::Frame;

pub struct Compositor {
    buffers: [CompositorBuffer; 2],
    active_buffer: usize,
    screen_size: (usize, usize),
}

enum CompositorBuffer {
    Rendered(CharBuffer),
    NotRendered(Frame),
}

impl Compositor {
    pub fn new(screen_size: (usize, usize)) -> Self {
        let buffers = [
            CompositorBuffer::default(),
            CompositorBuffer::default(),
        ];

        Self {
            buffers,
            active_buffer: 0,
            screen_size,
        }
    }

    /// renders the active buffer and returns it to the user,
    /// switches the active buffer
    pub fn render(&mut self) -> Result<CharBuffer> {
        //TODO: find the diff between the two buffers
        // and only render the diff

        let mut buffer = CharBuffer::new(self.screen_size.0, self.screen_size.1);

        self.buffers[self.active_buffer].render()?;
        self.switch_active_buffer();
        Ok()
    }

    fn switch_active_buffer(&mut self) {
        self.active_buffer = (self.active_buffer + 1) % 2;
    }

    /// accepts a closure that takes a frame and returns a full built frame as a result
    ///
    /// will always swith the compositor's active buffer to unrendered,
    /// deleting the previous buffer
    pub fn draw(&mut self, f: impl FnOnce(Frame) -> Result<Frame>) -> Result<()> {
        self.buffers[self.active_buffer]

        self.buffers[self.active_buffer] = frame.render()?;

        Ok(())
    }
}

impl CompositorBuffer {
    fn render(&mut self) -> Result<()> {
        match self {
            Self::Rendered(_) => Ok(()),
            Self::NotRendered(frame) => {
                *self = Self::Rendered(frame.render()?);
                Ok(())
            }
        }
    }
}

impl Default for CompositorBuffer {
    fn default() -> Self {
        Self::NotRendered(Frame::new())
    }
}
