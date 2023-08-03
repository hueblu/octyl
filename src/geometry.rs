#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Split {
    pub orientation: Orientation,
    pub ratio: Ratio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

// type that represents the ratio
// between two sections of a split
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ratio {
    Percentage(f32),
    // 50/50
    Fixed,
}

impl Direction {
    fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

impl Default for Ratio {
    fn default() -> Self {
        Ratio::Fixed
    }
}
