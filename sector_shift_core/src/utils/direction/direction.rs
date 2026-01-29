use std::{
    fmt::Display,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::direction::{DirectionFlags, DirectionIter};

/// Includes Directions [`NORTH`, `EAST`, `SOUTH`, `WEST`]
pub struct CardinalDirection;
impl CardinalDirection {
    /// Returns a [`Direction`] representing `EAST`
    pub const EAST: Direction = Direction::EAST;
    /// Returns a [`Direction`] representing `NORTH`
    pub const NORTH: Direction = Direction::NORTH;
    /// Returns a [`Direction`] representing `SOUTH`
    pub const SOUTH: Direction = Direction::SOUTH;
    /// Returns a [`Direction`] representing `WEST`
    pub const WEST: Direction = Direction::WEST;
    /// Returns a [`Direction`] representing the combination of all cardinal directions
    pub const MASK: Direction = Direction::CARDINAL_MASK;

    /// Returns an iterator over the [`Direction`]s [`NORTH`, `EAST`, `SOUTH`, `WEST`]
    pub const fn iter() -> DirectionIter {
        DirectionIter::cardinal()
    }
}

/// Includes Directions [`NORTH_EAST`, `SOUTH_EAST`, `SOUTH_WEST`, `NORTH_WEST`]
pub struct OrdinalDirection;
impl OrdinalDirection {
    /// Returns a [`Direction`] representing `NORTH_EAST`
    pub const NORTH_EAST: Direction = Direction::NORTH_EAST;
    /// Returns a [`Direction`] representing `NORTH_WEST`
    pub const NORTH_WEST: Direction = Direction::NORTH_WEST;
    /// Returns a [`Direction`] representing `SOUTH_EAST`
    pub const SOUTH_EAST: Direction = Direction::SOUTH_EAST;
    /// Returns a [`Direction`] representing `SOUTH_WEST`
    pub const SOUTH_WEST: Direction = Direction::SOUTH_WEST;

    /// Returns an iterator over the [`Direction`]s [`NORTH_EAST`, `SOUTH_EAST`, `SOUTH_WEST`,
    /// `NORTH_WEST`]
    pub const fn iter() -> DirectionIter {
        DirectionIter::ordinal()
    }
}

/// Includes Directions [`UP`, `DOWN`]
pub struct VerticalDirection;
impl VerticalDirection {
    /// Returns a [`Direction`] representing `DOWN`
    pub const DOWN: Direction = Direction::DOWN;
    /// Returns a [`Direction`] representing `UP`
    pub const UP: Direction = Direction::UP;
    /// Returns a [`Direction`] representing the combination of both vertical directions
    pub const MASK: Direction = Direction::VERTICAL_MASK;

    /// Returns an iterator over the [`Direction`]s [`UP`, `DOWN`]
    pub const fn iter() -> DirectionIter {
        DirectionIter::vertical()
    }
}

#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Direction(u8);

// Constants
impl Direction {
    /// Returns the `DOWN` [`Direction`]
    pub const DOWN: Self = Self(DirectionFlags::DOWN);
    /// Returns the `DOWN_EAST` [`Direction`]
    pub const DOWN_EAST: Self = Self(DirectionFlags::DOWN | DirectionFlags::EAST);
    /// Returns the `DOWN_NORTH` [`Direction`]
    pub const DOWN_NORTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::NORTH);
    /// Returns the `DOWN_NORTH_EAST` [`Direction`]
    pub const DOWN_NORTH_EAST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::EAST);
    /// Returns the `DOWN_NORTH_WEST` [`Direction`]
    pub const DOWN_NORTH_WEST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::NORTH | DirectionFlags::WEST);
    /// Returns the `DOWN_SOUTH` [`Direction`]
    pub const DOWN_SOUTH: Self = Self(DirectionFlags::DOWN | DirectionFlags::SOUTH);
    /// Returns the `DOWN_SOUTH_EAST` [`Direction`]
    pub const DOWN_SOUTH_EAST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::EAST);
    /// Returns the `DOWN_SOUTH_WEST` [`Direction`]
    pub const DOWN_SOUTH_WEST: Self =
        Self(DirectionFlags::DOWN | DirectionFlags::SOUTH | DirectionFlags::WEST);
    /// Returns the `DOWN_WEST` [`Direction`]
    pub const DOWN_WEST: Self = Self(DirectionFlags::DOWN | DirectionFlags::WEST);
    /// Returns the `EAST` [`Direction`]
    pub const EAST: Self = Self(DirectionFlags::EAST);
    /// Returns the `NO DIRECTION` [`Direction`]
    pub const NONE: Self = Self(0);
    /// Returns the `NORTH` [`Direction`]
    pub const NORTH: Self = Self(DirectionFlags::NORTH);
    /// Returns the `NORTH_EAST` [`Direction`]
    pub const NORTH_EAST: Self = Self(DirectionFlags::NORTH | DirectionFlags::EAST);
    /// Returns the `NORTH_WEST` [`Direction`]
    pub const NORTH_WEST: Self = Self(DirectionFlags::NORTH | DirectionFlags::WEST);
    /// Returns the `SOUTH` [`Direction`]
    pub const SOUTH: Self = Self(DirectionFlags::SOUTH);
    /// Returns the `SOUTH_EAST` [`Direction`]
    pub const SOUTH_EAST: Self = Self(DirectionFlags::SOUTH | DirectionFlags::EAST);
    /// Returns the `SOUTH_WEST` [`Direction`]
    pub const SOUTH_WEST: Self = Self(DirectionFlags::SOUTH | DirectionFlags::WEST);
    /// Returns the `UP` [`Direction`]
    pub const UP: Self = Self(DirectionFlags::UP);
    /// Returns the `UP_EAST` [`Direction`]
    pub const UP_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::EAST);
    /// Returns the `UP_NORTH` [`Direction`]
    pub const UP_NORTH: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH);
    /// Returns the `UP_NORTH_EAST` [`Direction`]
    pub const UP_NORTH_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::EAST);
    /// Returns the `UP_NORTH_WEST` [`Direction`]
    pub const UP_NORTH_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::NORTH | DirectionFlags::WEST);
    /// Returns the `UP_SOUTH` [`Direction`]
    pub const UP_SOUTH: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH);
    /// Returns the `UP_SOUTH_EAST` [`Direction`]
    pub const UP_SOUTH_EAST: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::EAST);
    /// Returns the `UP_SOUTH_WEST` [`Direction`]
    pub const UP_SOUTH_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::SOUTH | DirectionFlags::WEST);
    /// Returns the `UP_WEST` [`Direction`]
    pub const UP_WEST: Self = Self(DirectionFlags::UP | DirectionFlags::WEST);
    /// Returns the `WEST` [`Direction`]
    pub const WEST: Self = Self(DirectionFlags::WEST);
    /// Returns a [`Direction`] representing the combination of all cardinal directions
    pub const CARDINAL_MASK: Self =
        Self(DirectionFlags::NORTH | DirectionFlags::EAST | DirectionFlags::SOUTH | DirectionFlags::WEST);
    /// Returns a [`Direction`] representing the combination of all vertical directions
    pub const VERTICAL_MASK: Self = Self(DirectionFlags::UP | DirectionFlags::DOWN);
}

// To-From Coord
impl Direction {
    /// Retrieves the 2d coordinate value from the [`Direction`] where:
    ///
    /// `EAST` = `1` on the `X` axis
    ///
    /// `WEST` = `-1` on the `X` axis
    ///
    /// `NORTH` = `1` on the `Y` axis
    ///
    /// `SOUTH` = `-1` on the `Y` axis
    #[inline]
    pub const fn coord(self) -> (i32, i32) {
        let x = (self.has_east() as i32) - (self.has_west() as i32);
        let y = (self.has_north() as i32) - (self.has_south() as i32);
        (x, y)
    }

    /// Retrieves the 3d coordinate value from the [`Direction`] where:
    ///
    /// `EAST` = `1` on the `X` axis
    ///
    /// `WEST` = `-1` on the `X` axis
    ///
    /// `NORTH` = `1` on the `Y` axis
    ///
    /// `SOUTH` = `-1` on the `Y` axis
    ///
    /// `UP` = `1` on the `Z` axis
    ///
    /// `DOWN` = `-1` on the `Z` axis
    #[inline]
    pub const fn coord3d(self) -> (i32, i32, i32) {
        let x = (self.has_east() as i32) - (self.has_west() as i32);
        let y = (self.has_north() as i32) - (self.has_south() as i32);
        let z = (self.has_up() as i32) - (self.has_down() as i32);
        (x, y, z)
    }

    /// Retrieves the [`Direction`] from a 2d coordinate value where:
    ///
    /// `EAST` = `1` on the `X` axis
    ///
    /// `WEST` = `-1` on the `X` axis
    ///
    /// `NORTH` = `1` on the `Y` axis
    ///
    /// `SOUTH` = `-1` on the `Y` axis
    pub fn from_coord(coord: (i32, i32)) -> Self {
        let (x, y) = coord;
        Self::from_coord3d((x, y, 0))
    }

    /// Retrieves the [`Direction`] from a 3d coordinate value where:
    ///
    /// `EAST` = `1` on the `X` axis
    ///
    /// `WEST` = `-1` on the `X` axis
    ///
    /// `NORTH` = `1` on the `Y` axis
    ///
    /// `SOUTH` = `-1` on the `Y` axis
    ///
    /// `UP` = `1` on the `Z` axis
    ///
    /// `DOWN` = `-1` on the `Z` axis
    pub fn from_coord3d(coord: (i32, i32, i32)) -> Self {
        let mut direction = Self::NONE;

        // Check X
        match coord.0.cmp(&0) {
            std::cmp::Ordering::Less => direction += Self::WEST,
            std::cmp::Ordering::Equal => {},
            std::cmp::Ordering::Greater => direction += Self::EAST,
        }

        // Check Y
        match coord.1.cmp(&0) {
            std::cmp::Ordering::Less => direction += Self::SOUTH,
            std::cmp::Ordering::Equal => {},
            std::cmp::Ordering::Greater => direction += Self::NORTH,
        }

        // Check Z
        match coord.2.cmp(&0) {
            std::cmp::Ordering::Less => direction += Self::DOWN,
            std::cmp::Ordering::Equal => {},
            std::cmp::Ordering::Greater => direction += Self::UP,
        }

        direction
    }
}

// Rotations
impl Direction {
    /// Rotates a [`Direction`] counter clockwise by one step:
    ///
    /// `NORTH_EAST` becomes `NORTH`
    ///
    /// `NORTH` becomes `NORTH_WEST`
    ///
    /// etc...
    pub fn left45(self) -> Self {
        let rotated = self.left90();
        let horizontal_part = self.0 & CardinalDirection::MASK.0;

        if horizontal_part.count_ones() == 1 { self | rotated } else { self & rotated }
    }

    /// Rotates a [`Direction`] counter clockwise by two steps:
    ///
    /// `NORTH_EAST` becomes `NORTH_WEST`
    ///
    /// `NORTH` becomes `WEST`
    ///
    /// etc...
    pub fn left90(self) -> Self {
        let mut new_flags = 0;
        if self.has_up() {
            new_flags |= DirectionFlags::UP;
        }
        if self.has_down() {
            new_flags |= DirectionFlags::DOWN;
        }

        if self.has_north() {
            new_flags |= DirectionFlags::WEST;
        }
        if self.has_east() {
            new_flags |= DirectionFlags::NORTH;
        }
        if self.has_south() {
            new_flags |= DirectionFlags::EAST;
        }
        if self.has_west() {
            new_flags |= DirectionFlags::SOUTH;
        }

        Self(new_flags)
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] counter clockwise by three steps:
    ///
    /// `NORTH_EAST` becomes `WEST`
    ///
    /// `NORTH` becomes `SOUTH_WEST`
    ///
    /// etc...
    pub fn left135(self) -> Self {
        self.left90().left45()
    }

    /// Retrieves the opposite (rotated by four steps) [`Direction`] from another [`Direction`]
    /// where:
    ///
    /// `NORTH` and `SOUTH` are opposite:
    ///
    /// `EAST` and `WEST` are opposite:
    ///
    /// `UP` and `DOWN` are opposite:
    pub fn opposite(self) -> Self {
        let (x, y, z) = self.coord3d();
        Self::from_coord3d((-x, -y, -z))
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] clockwise by three steps:
    ///
    /// `NORTH_EAST` becomes `SOUTH`
    ///
    /// `NORTH` becomes `SOUTH_EAST`
    ///
    /// etc...
    pub fn right135(self) -> Self {
        self.right90().right45()
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] clockwise by two steps:
    ///
    /// `NORTH_EAST` becomes `SOUTH_EAST`
    ///
    /// `NORTH` becomes `EAST`
    ///
    /// etc...
    pub fn right90(self) -> Self {
        let mut new_flags = 0;
        if self.has_up() {
            new_flags |= DirectionFlags::UP;
        }
        if self.has_down() {
            new_flags |= DirectionFlags::DOWN;
        }

        if self.has_north() {
            new_flags |= DirectionFlags::EAST;
        }
        if self.has_east() {
            new_flags |= DirectionFlags::SOUTH;
        }
        if self.has_south() {
            new_flags |= DirectionFlags::WEST;
        }
        if self.has_west() {
            new_flags |= DirectionFlags::NORTH;
        }

        Self(new_flags)
    }

    // OPTIMIZE: rewrite like `Self::left45()`
    /// Rotates a [`Direction`] clockwise by one step:
    ///
    /// `NORTH_EAST` becomes `EAST`
    ///
    /// `NORTH` becomes `NORTH_EAST`
    ///
    /// etc...
    pub fn right45(self) -> Self {
        let rotated = self.right90();
        let horizontal_part = self.0 & CardinalDirection::MASK.0;

        if horizontal_part.count_ones() == 1 { self | rotated } else { self & rotated }
    }
}

// Checks
impl Direction {
    /// Checks a [`Direction`] to determine if it is marked as `NORTH`
    ///
    /// Returns: `true` if [`Direction`] is marked `NORTH`
    #[inline]
    pub const fn has_north(self) -> bool {
        self.0 & Self::NORTH.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `SOUTH`
    ///
    /// Returns: `true` if [`Direction`] is marked `SOUTH`
    #[inline]
    pub const fn has_south(self) -> bool {
        self.0 & Self::SOUTH.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `EAST`
    ///
    /// Returns: `true` if [`Direction`] is marked `EAST`
    #[inline]
    pub const fn has_east(self) -> bool {
        self.0 & Self::EAST.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `WEST`
    ///
    /// Returns: `true` if [`Direction`] is marked `WEST`
    #[inline]
    pub const fn has_west(self) -> bool {
        self.0 & Self::WEST.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `UP`
    ///
    /// Returns: `true` if [`Direction`] is marked `UP`
    #[inline]
    pub const fn has_up(self) -> bool {
        self.0 & Self::UP.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is marked as `DOWN`
    ///
    /// Returns: `true` if [`Direction`] is marked `DOWN`
    #[inline]
    pub const fn has_down(self) -> bool {
        self.0 & Self::DOWN.0 != 0
    }

    /// Checks a [`Direction`] to determine if it is a [`CardinalDirection`]
    ///
    /// Returns: `true` if [`Direction`] contains a [`CardinalDirection`]
    /// NOTE: This function has no reguard for `UP` or `DOWN` therefore `Direction::UP_NORTH`
    /// will return `true`
    pub const fn is_cardinal(self) -> bool {
        (self.has_north() || self.has_south()) && !(self.has_east() || self.has_west())
            || (self.has_east() || self.has_west()) && !(self.has_north() || self.has_south())
    }

    /// Checks a [`Direction`] to determine if it is a [`OrdinalDirection`]
    ///
    /// Returns: `true` if [`Direction`] contains a [`OrdinalDirection`]
    /// NOTE: This function has no reguard for `UP` or `DOWN` therefore
    /// `Direction::UP_NORTH_EAST` will return `true`
    pub const fn is_ordinal(self) -> bool {
        (self.has_north() || self.has_south()) && (self.has_east() || self.has_west())
    }

    pub fn simplify(&mut self) {
        if self.has_north() && self.has_south() {
            *self -= Self::NORTH;
            *self -= Self::SOUTH;
        }

        if self.has_east() && self.has_west() {
            *self -= Self::EAST;
            *self -= Self::WEST;
        }

        if self.has_up() && self.has_down() {
            *self -= Self::UP;
            *self -= Self::DOWN;
        }
    }
}

// Iterators
impl Direction {
    /// Returns an iterator over the [`Direction`]s [`NORTH`, `NORTH_EAST`, `EAST`,
    /// `SOUTH_EAST`, `SOUTH`, `SOUTH_WEST`, `WEST`, `NORTH_WEST`]
    pub const fn iter_cardinal_ordinal() -> DirectionIter {
        DirectionIter::cardinal_ordinal()
    }

    /// Returns an iterator over the [`Direction`]s [`NORTH`, `NORTH_EAST`, `EAST`,
    /// `SOUTH_EAST`, `SOUTH`, `SOUTH_WEST`, `WEST`, `NORTH_WEST`, `UP`, `DOWN`]
    pub const fn iter_cardinal_ordinal_vertical() -> DirectionIter {
        DirectionIter::cardinal_ordinal_vertical()
    }

    /// Returns an iterator over every [`Direction`] including every mutation of [`NORTH`,
    /// `EAST`, `SOUTH`, `WEST`, `UP`, `DOWN`]
    pub const fn iter_all_3d() -> DirectionIter {
        DirectionIter::all_3d()
    }

    /// Returns an iterator over the [`Direction`]s [`NORTH`, `EAST`, `SOUTH`, `WEST`]
    pub const fn iter_cardinal() -> DirectionIter {
        DirectionIter::cardinal()
    }

    /// Returns an iterator over the [`Direction`]s [`NORTH_EAST`, `SOUTH_EAST`, `SOUTH_WEST`,
    /// `NORTH_WEST`]
    pub const fn iter_ordinal() -> DirectionIter {
        DirectionIter::ordinal()
    }

    /// Returns an iterator over the [`Direction`]s (`UP`, `DOWN`)
    pub const fn iter_vertical() -> DirectionIter {
        DirectionIter::vertical()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn append(old_string: String, next: &str, first: bool) -> String {
            if first { next.to_string() } else { format!("{old_string}, {next}") }
        }

        let mut s = String::new();
        let mut first = true;

        if self.has_north() {
            s = append(s, "NORTH", first);
            first = false;
        }

        if self.has_east() {
            s = append(s, "EAST", first);
            first = false;
        }

        if self.has_south() {
            s = append(s, "SOUTH", first);
            first = false;
        }

        if self.has_west() {
            s = append(s, "WEST", first);
            first = false;
        }

        if self.has_up() {
            s = append(s, "UP", first);
            first = false;
        }

        if self.has_down() {
            s = append(s, "DOWN", first);
            first = false;
        }

        if first { write!(f, "NO-DIRECTION") } else { write!(f, "{s}") }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Self> for Direction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Self> for Direction {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Sub<Self> for Direction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl SubAssign<Self> for Direction {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 &= !rhs.0
    }
}

impl BitOr<Self> for Direction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign<Self> for Direction {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd<Self> for Direction {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign<Self> for Direction {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor<Self> for Direction {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<Self> for Direction {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::{CardinalDirection, Direction, OrdinalDirection, VerticalDirection};

    #[test]
    fn test_direction_const() {
        // Match these to DirectionFlags
        const NORTH: u8 = 1 << 0;
        const EAST: u8 = 1 << 1;
        const SOUTH: u8 = 1 << 2;
        const WEST: u8 = 1 << 3;
        const UP: u8 = 1 << 4;
        const DOWN: u8 = 1 << 5;

        // Test creation using constants
        assert_eq!(Direction::NORTH.0, NORTH);
        assert_eq!(Direction::NORTH_EAST.0, NORTH + EAST);
        assert_eq!(Direction::EAST.0, EAST);
        assert_eq!(Direction::SOUTH_EAST.0, SOUTH + EAST);
        assert_eq!(Direction::SOUTH.0, SOUTH);
        assert_eq!(Direction::SOUTH_WEST.0, SOUTH + WEST);
        assert_eq!(Direction::WEST.0, WEST);
        assert_eq!(Direction::NORTH_WEST.0, NORTH + WEST);
        assert_eq!(Direction::UP.0, UP);
        assert_eq!(Direction::UP_NORTH.0, UP + NORTH);
        assert_eq!(Direction::UP_NORTH_EAST.0, UP + NORTH + EAST);
        assert_eq!(Direction::UP_EAST.0, UP + EAST);
        assert_eq!(Direction::UP_SOUTH_EAST.0, UP + SOUTH + EAST);
        assert_eq!(Direction::UP_SOUTH.0, UP + SOUTH);
        assert_eq!(Direction::UP_SOUTH_WEST.0, UP + SOUTH + WEST);
        assert_eq!(Direction::UP_WEST.0, UP + WEST);
        assert_eq!(Direction::UP_NORTH_WEST.0, UP + NORTH + WEST);
        assert_eq!(Direction::DOWN.0, DOWN);
        assert_eq!(Direction::DOWN_NORTH.0, DOWN + NORTH);
        assert_eq!(Direction::DOWN_NORTH_EAST.0, DOWN + NORTH + EAST);
        assert_eq!(Direction::DOWN_EAST.0, DOWN + EAST);
        assert_eq!(Direction::DOWN_SOUTH_EAST.0, DOWN + SOUTH + EAST);
        assert_eq!(Direction::DOWN_SOUTH.0, DOWN + SOUTH);
        assert_eq!(Direction::DOWN_SOUTH_WEST.0, DOWN + SOUTH + WEST);
        assert_eq!(Direction::DOWN_WEST.0, DOWN + WEST);
        assert_eq!(Direction::DOWN_NORTH_WEST.0, DOWN + NORTH + WEST);

        assert_eq!(CardinalDirection::NORTH, Direction::NORTH);
        assert_eq!(CardinalDirection::EAST, Direction::EAST);
        assert_eq!(CardinalDirection::SOUTH, Direction::SOUTH);
        assert_eq!(CardinalDirection::WEST, Direction::WEST);

        assert_eq!(OrdinalDirection::NORTH_EAST, Direction::NORTH_EAST);
        assert_eq!(OrdinalDirection::NORTH_WEST, Direction::NORTH_WEST);
        assert_eq!(OrdinalDirection::SOUTH_EAST, Direction::SOUTH_EAST);
        assert_eq!(OrdinalDirection::SOUTH_WEST, Direction::SOUTH_WEST);

        assert_eq!(VerticalDirection::UP, Direction::UP);
        assert_eq!(VerticalDirection::DOWN, Direction::DOWN);
    }

    #[test]
    fn test_direction_coord() {
        // Test creation from coords
        assert_eq!(Direction::from_coord((0, 1)), Direction::NORTH);
        assert_eq!(Direction::from_coord((1, 1)), Direction::NORTH_EAST);
        assert_eq!(Direction::from_coord((1, 0)), Direction::EAST);
        assert_eq!(Direction::from_coord((1, -1)), Direction::SOUTH_EAST);
        assert_eq!(Direction::from_coord((0, -1)), Direction::SOUTH);
        assert_eq!(Direction::from_coord((-1, -1)), Direction::SOUTH_WEST);
        assert_eq!(Direction::from_coord((-1, 0)), Direction::WEST);
        assert_eq!(Direction::from_coord((-1, 1)), Direction::NORTH_WEST);

        // Test to coords
        assert_eq!(Direction::UP_NORTH.coord(), (0, 1));
        assert_eq!(Direction::UP_NORTH_EAST.coord(), (1, 1));
        assert_eq!(Direction::UP_EAST.coord(), (1, 0));
        assert_eq!(Direction::UP_SOUTH_EAST.coord(), (1, -1));
        assert_eq!(Direction::UP_SOUTH.coord(), (0, -1));
        assert_eq!(Direction::UP_SOUTH_WEST.coord(), (-1, -1));
        assert_eq!(Direction::UP_WEST.coord(), (-1, 0));
        assert_eq!(Direction::UP_NORTH_WEST.coord(), (-1, 1));

        assert_eq!(Direction::NORTH.coord(), (0, 1));
        assert_eq!(Direction::NORTH_EAST.coord(), (1, 1));
        assert_eq!(Direction::EAST.coord(), (1, 0));
        assert_eq!(Direction::SOUTH_EAST.coord(), (1, -1));
        assert_eq!(Direction::SOUTH.coord(), (0, -1));
        assert_eq!(Direction::SOUTH_WEST.coord(), (-1, -1));
        assert_eq!(Direction::WEST.coord(), (-1, 0));
        assert_eq!(Direction::NORTH_WEST.coord(), (-1, 1));

        assert_eq!(Direction::DOWN_NORTH.coord(), (0, 1));
        assert_eq!(Direction::DOWN_NORTH_EAST.coord(), (1, 1));
        assert_eq!(Direction::DOWN_EAST.coord(), (1, 0));
        assert_eq!(Direction::DOWN_SOUTH_EAST.coord(), (1, -1));
        assert_eq!(Direction::DOWN_SOUTH.coord(), (0, -1));
        assert_eq!(Direction::DOWN_SOUTH_WEST.coord(), (-1, -1));
        assert_eq!(Direction::DOWN_WEST.coord(), (-1, 0));
        assert_eq!(Direction::DOWN_NORTH_WEST.coord(), (-1, 1));

        // Test creation from coords 3d
        assert_eq!(Direction::from_coord3d((0, 1, 1)), Direction::UP_NORTH);
        assert_eq!(Direction::from_coord3d((1, 1, 1)), Direction::UP_NORTH_EAST);
        assert_eq!(Direction::from_coord3d((1, 0, 1)), Direction::UP_EAST);
        assert_eq!(
            Direction::from_coord3d((1, -1, 1)),
            Direction::UP_SOUTH_EAST
        );
        assert_eq!(Direction::from_coord3d((0, -1, 1)), Direction::UP_SOUTH);
        assert_eq!(
            Direction::from_coord3d((-1, -1, 1)),
            Direction::UP_SOUTH_WEST
        );
        assert_eq!(Direction::from_coord3d((-1, 0, 1)), Direction::UP_WEST);
        assert_eq!(
            Direction::from_coord3d((-1, 1, 1)),
            Direction::UP_NORTH_WEST
        );

        assert_eq!(Direction::from_coord3d((0, 1, 0)), Direction::NORTH);
        assert_eq!(Direction::from_coord3d((1, 1, 0)), Direction::NORTH_EAST);
        assert_eq!(Direction::from_coord3d((1, 0, 0)), Direction::EAST);
        assert_eq!(Direction::from_coord3d((1, -1, 0)), Direction::SOUTH_EAST);
        assert_eq!(Direction::from_coord3d((0, -1, 0)), Direction::SOUTH);
        assert_eq!(Direction::from_coord3d((-1, -1, 0)), Direction::SOUTH_WEST);
        assert_eq!(Direction::from_coord3d((-1, 0, 0)), Direction::WEST);
        assert_eq!(Direction::from_coord3d((-1, 1, 0)), Direction::NORTH_WEST);

        assert_eq!(Direction::from_coord3d((0, 1, -1)), Direction::DOWN_NORTH);
        assert_eq!(
            Direction::from_coord3d((1, 1, -1)),
            Direction::DOWN_NORTH_EAST
        );
        assert_eq!(Direction::from_coord3d((1, 0, -1)), Direction::DOWN_EAST);
        assert_eq!(
            Direction::from_coord3d((1, -1, -1)),
            Direction::DOWN_SOUTH_EAST
        );
        assert_eq!(Direction::from_coord3d((0, -1, -1)), Direction::DOWN_SOUTH);
        assert_eq!(
            Direction::from_coord3d((-1, -1, -1)),
            Direction::DOWN_SOUTH_WEST
        );
        assert_eq!(Direction::from_coord3d((-1, 0, -1)), Direction::DOWN_WEST);
        assert_eq!(
            Direction::from_coord3d((-1, 1, -1)),
            Direction::DOWN_NORTH_WEST
        );

        // Test to coords 3d
        assert_eq!(Direction::UP_NORTH.coord3d(), (0, 1, 1));
        assert_eq!(Direction::UP_NORTH_EAST.coord3d(), (1, 1, 1));
        assert_eq!(Direction::UP_EAST.coord3d(), (1, 0, 1));
        assert_eq!(Direction::UP_SOUTH_EAST.coord3d(), (1, -1, 1));
        assert_eq!(Direction::UP_SOUTH.coord3d(), (0, -1, 1));
        assert_eq!(Direction::UP_SOUTH_WEST.coord3d(), (-1, -1, 1));
        assert_eq!(Direction::UP_WEST.coord3d(), (-1, 0, 1));
        assert_eq!(Direction::UP_NORTH_WEST.coord3d(), (-1, 1, 1));

        assert_eq!(Direction::NORTH.coord3d(), (0, 1, 0));
        assert_eq!(Direction::NORTH_EAST.coord3d(), (1, 1, 0));
        assert_eq!(Direction::EAST.coord3d(), (1, 0, 0));
        assert_eq!(Direction::SOUTH_EAST.coord3d(), (1, -1, 0));
        assert_eq!(Direction::SOUTH.coord3d(), (0, -1, 0));
        assert_eq!(Direction::SOUTH_WEST.coord3d(), (-1, -1, 0));
        assert_eq!(Direction::WEST.coord3d(), (-1, 0, 0));
        assert_eq!(Direction::NORTH_WEST.coord3d(), (-1, 1, 0));

        assert_eq!(Direction::DOWN_NORTH.coord3d(), (0, 1, -1));
        assert_eq!(Direction::DOWN_NORTH_EAST.coord3d(), (1, 1, -1));
        assert_eq!(Direction::DOWN_EAST.coord3d(), (1, 0, -1));
        assert_eq!(Direction::DOWN_SOUTH_EAST.coord3d(), (1, -1, -1));
        assert_eq!(Direction::DOWN_SOUTH.coord3d(), (0, -1, -1));
        assert_eq!(Direction::DOWN_SOUTH_WEST.coord3d(), (-1, -1, -1));
        assert_eq!(Direction::DOWN_WEST.coord3d(), (-1, 0, -1));
        assert_eq!(Direction::DOWN_NORTH_WEST.coord3d(), (-1, 1, -1));
    }

    #[test]
    fn test_direction_rotation() {
        // Test left rotation
        assert_eq!(Direction::UP_NORTH.left45(), Direction::UP_NORTH_WEST);
        assert_eq!(Direction::UP_NORTH.left90(), Direction::UP_WEST);
        assert_eq!(Direction::UP_NORTH.left135(), Direction::UP_SOUTH_WEST);

        assert_eq!(Direction::NORTH.left45(), Direction::NORTH_WEST);
        assert_eq!(Direction::NORTH.left90(), Direction::WEST);
        assert_eq!(Direction::NORTH.left135(), Direction::SOUTH_WEST);

        assert_eq!(Direction::DOWN_NORTH.left45(), Direction::DOWN_NORTH_WEST);
        assert_eq!(Direction::DOWN_NORTH.left90(), Direction::DOWN_WEST);
        assert_eq!(Direction::DOWN_NORTH.left135(), Direction::DOWN_SOUTH_WEST);

        // Test right rotation
        assert_eq!(Direction::UP_NORTH.right45(), Direction::UP_NORTH_EAST);
        assert_eq!(Direction::UP_NORTH.right90(), Direction::UP_EAST);
        assert_eq!(Direction::UP_NORTH.right135(), Direction::UP_SOUTH_EAST);

        assert_eq!(Direction::NORTH.right45(), Direction::NORTH_EAST);
        assert_eq!(Direction::NORTH.right90(), Direction::EAST);
        assert_eq!(Direction::NORTH.right135(), Direction::SOUTH_EAST);

        assert_eq!(Direction::DOWN_NORTH.right45(), Direction::DOWN_NORTH_EAST);
        assert_eq!(Direction::DOWN_NORTH.right90(), Direction::DOWN_EAST);
        assert_eq!(Direction::DOWN_NORTH.right135(), Direction::DOWN_SOUTH_EAST);

        // Test opposite
        assert_eq!(Direction::NORTH.opposite(), Direction::SOUTH);
        assert_eq!(Direction::EAST.opposite(), Direction::WEST);
        assert_eq!(Direction::UP.opposite(), Direction::DOWN);
    }

    #[test]
    fn test_direction_iterators() {
        let cardinal_directions: Vec<Direction> = CardinalDirection::iter().collect();
        assert_eq!(
            cardinal_directions,
            vec![
                Direction::NORTH,
                Direction::EAST,
                Direction::SOUTH,
                Direction::WEST
            ]
        );
        assert_eq!(
            cardinal_directions,
            Direction::iter_cardinal().collect::<Vec<Direction>>()
        );

        let ordinal_directions: Vec<Direction> = OrdinalDirection::iter().collect();
        assert_eq!(
            ordinal_directions,
            vec![
                Direction::NORTH_EAST,
                Direction::SOUTH_EAST,
                Direction::SOUTH_WEST,
                Direction::NORTH_WEST
            ]
        );
        assert_eq!(
            ordinal_directions,
            Direction::iter_ordinal().collect::<Vec<Direction>>()
        );

        let vertical_directions: Vec<Direction> = VerticalDirection::iter().collect();
        assert_eq!(vertical_directions, vec![Direction::UP, Direction::DOWN]);
        assert_eq!(
            vertical_directions,
            Direction::iter_vertical().collect::<Vec<Direction>>()
        );

        assert_eq!(Direction::iter_cardinal_ordinal().count(), 8);
        assert_eq!(Direction::iter_cardinal_ordinal_vertical().count(), 10);
        assert_eq!(Direction::iter_all_3d().count(), 26);
    }
}
