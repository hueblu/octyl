#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    corner1: Position,
    corner2: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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

impl Rect {
    pub fn new(corner1: Position, corner2: Position) -> Self {
        Self { corner1, corner2 }
    }

    pub fn width(&self) -> usize {
        self.corner2.x - self.corner1.x
    }

    pub fn height(&self) -> usize {
        self.corner2.y - self.corner1.y
    }
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

impl Position {
    pub fn area(self) -> usize {
        self.x * self.y
    }
}

impl From<(u16, u16)> for Position {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0 as usize,
            y: value.1 as usize,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Default for Ratio {
    fn default() -> Self {
        Ratio::Fixed
    }
}
