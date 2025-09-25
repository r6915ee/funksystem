/// Defines a pair of coordinates.
///
/// Typically, components such as [Position] will implement this trait, because
/// it allows access to coordinates. This is particularly useful for graphics,
/// because it allows interactions through motion.
pub trait Vector2<T> {
    /// Constructs a vector at the origin.
    fn new() -> Self;
    /// Constructs a vector with certain coordinates.
    fn spawn(x: T, y: T) -> Self;
    /// Gets the x coordinate of the vector.
    fn get_x(&self) -> T;
    /// Gets the y coordinate of the vector.
    fn get_y(&self) -> T;
}

/// Defines the position for a certain graphic.
#[allow(dead_code)] // TODO: remove once fields will be read
pub struct Position(pub u16, pub u16);

impl Vector2<u16> for Position {
    fn new() -> Position {
        Position(0, 0)
    }
    fn spawn(x: u16, y: u16) -> Position {
        Position(x, y)
    }
    fn get_x(&self) -> u16 {
        self.0
    }
    fn get_y(&self) -> u16 {
        self.1
    }
}

/// Defines the scale for a certain graphic.
#[allow(dead_code)] // TODO: remove once fields will be read
pub struct Scale(pub u8, pub u8);

impl Vector2<u8> for Scale {
    fn new() -> Scale {
        Scale(0, 0)
    }
    fn spawn(x: u8, y: u8) -> Scale {
        Scale(x, y)
    }
    fn get_x(&self) -> u8 {
        self.0
    }
    fn get_y(&self) -> u8 {
        self.1
    }
}
