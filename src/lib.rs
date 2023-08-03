// compositor that handles the rendering
// of components to the screen
pub mod compositor;

// document that holds text and
// cursor position
pub mod document;

// error handling
pub mod error;

// geometry and layout shit
pub mod geometry;

// utility
pub mod util;

pub use error::*;
pub use util::*;