pub mod component;
pub mod frame;

use crate::{geometry::Position, CharBuffer, Result};
pub use component::Component;
pub use frame::Frame;

pub struct Compositor {
    buffers: [CompositorBuffer; 2],
    active_buffer: usize,
    screen_size: Position,
}

enum CompositorBuffer {
    Rendered(CharBuffer),
    NotRendered(Frame),
}

impl Compositor {
    pub fn new(screen_size: Position) -> Self {
        let buffers = [
            CompositorBuffer::new(screen_size),
            CompositorBuffer::new(screen_size),
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

        self.active_buffer_mut().render();
        if let Some(buf) = self.active_buffer().get_char_buffer() {
            self.switch_active_buffer();
            Ok(buf)
        } else {
            unreachable!()
        }
    }

    fn switch_active_buffer(&mut self) {
        self.active_buffer = (self.active_buffer + 1) % 2;
    }

    /// accepts a closure that takes a frame and returns a full built frame as a result
    ///
    /// will always swith the compositor's active buffer to unrendered,
    /// deleting the previous buffer
    pub fn draw(&mut self, f: impl FnOnce(&mut Frame) -> Result<()>) -> Result<()> {
        let mut frame = Frame::new(self.screen_size);
        f(&mut frame)?;

        self.buffers[self.active_buffer] = CompositorBuffer::NotRendered(frame);

        Ok(())
    }

    fn active_buffer(&self) -> &CompositorBuffer {
        &self.buffers[self.active_buffer]
    }

    fn inactive_buffer(&self) -> &CompositorBuffer {
        &self.buffers[(self.active_buffer + 1) % 2]
    }

    fn active_buffer_mut(&mut self) -> &mut CompositorBuffer {
        &mut self.buffers[self.active_buffer]
    }

    fn inactive_buffer_mut(&mut self) -> &mut CompositorBuffer {
        &mut self.buffers[(self.active_buffer + 1) % 2]
    }
}

impl CompositorBuffer {
    fn new(screen_size: Position) -> Self {
        Self::NotRendered(Frame::new(screen_size))
    }

    fn render(&mut self) {
        match self {
            Self::Rendered(_) => (),
            Self::NotRendered(frame) => {
                *self = Self::Rendered(frame.render());
            }
        }
    }

    fn is_rendered(&self) -> bool {
        matches!(self, Self::Rendered(_))
    }

    fn get_char_buffer(&self) -> Option<CharBuffer> {
        match self {
            Self::Rendered(buffer) => Some(buffer.clone()),
            Self::NotRendered(_) => None,
        }
    }
}
