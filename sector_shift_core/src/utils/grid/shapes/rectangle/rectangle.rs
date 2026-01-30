use std::fmt::Display;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::grid::shapes::{
    BoxedShape, BoxedShapeIter, Shape, ShapeWithBorder,
    rectangle::{RectangleBorderIter, RectangleIter},
};

/// A 2D rectangle aligned to the grid axis.
///
/// Note: This uses **Exclusive Upper Bounds**.
/// A rectangle from (0,0) to (1,1) has a width of 1 and contains only the point (0,0).
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Rectangle {
    /// The minimum coordinates (inclusive).
    pub min: (i32, i32),
    /// The maximum coordinates (exclusive).
    pub max: (i32, i32),
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new_with_size((0, 0), (0, 0))
    }
}

// Constructors
impl Rectangle {
    /// Creates a new rectangle defined by two corners.
    /// Automatically sorts coordinates so min is top-left and max is bottom-right.
    #[inline]
    pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        Self {
            min: (p1.0.min(p2.0), p1.1.min(p2.1)),
            max: (p1.0.max(p2.0), p1.1.max(p2.1)),
        }
    }

    /// Creates a new rectangle from a top-left origin and size.
    #[inline]
    pub fn new_with_size(origin: (i32, i32), size: (u32, u32)) -> Self {
        Self {
            min: origin,
            max: (origin.0 + size.0 as i32, origin.1 + size.1 as i32),
        }
    }

    /// Creates a rectangle centered on a point with a given radius.
    /// Radius 0 = 1x1 point. Radius 1 = 3x3 square.
    pub fn from_center(center: (i32, i32), radius: u32) -> Self {
        let r = radius as i32;
        Self::new(
            (center.0 - r, center.1 - r),
            (center.0 + r + 1, center.1 + r + 1),
        )
    }
}

// Geometry
impl Rectangle {
    #[inline]
    pub const fn width(&self) -> i32 {
        self.max.0 - self.min.0
    }

    #[inline]
    pub const fn height(&self) -> i32 {
        self.max.1 - self.min.1
    }

    #[inline]
    pub const fn is_square(&self) -> bool {
        self.width() == self.height()
    }

    #[inline]
    pub const fn center(&self) -> (i32, i32) {
        (
            self.min.0 + self.width() / 2,
            self.min.1 + self.height() / 2,
        )
    }

    /// Returns true if the two rectangles overlap.
    /// Uses exclusive upper-bound logic.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        self.min.0 < other.max.0
            && self.max.0 > other.min.0
            && self.min.1 < other.max.1
            && self.max.1 > other.min.1
    }

    /// Returns the intersection of two rectangles (clamped).
    /// Returns a 0-size rectangle if they don't intersect.
    pub fn intersection(&self, other: Self) -> Self {
        if !self.intersects(other) {
            return Self::new(self.min, self.min);
        }
        let min_x = self.min.0.max(other.min.0);
        let min_y = self.min.1.max(other.min.1);
        let max_x = self.max.0.min(other.max.0);
        let max_y = self.max.1.min(other.max.1);
        Self::new((min_x, min_y), (max_x, max_y))
    }
}

// Trait Impls
impl Shape for Rectangle {
    #[inline]
    fn count(&self) -> usize {
        (self.width() as usize) * (self.height() as usize)
    }

    #[inline]
    fn contains(&self, position: (i32, i32)) -> bool {
        position.0 >= self.min.0
            && position.0 < self.max.0
            && position.1 >= self.min.1
            && position.1 < self.max.1
    }

    #[inline]
    fn iter(&'_ self) -> BoxedShapeIter<'_> {
        Box::new(RectangleIter::new(self.min, self.max))
    }
}

impl ShapeWithBorder for Rectangle {
    #[inline]
    fn border_count(&self) -> usize {
        let w = self.width() as usize;
        let h = self.height() as usize;
        if w == 0 || h == 0 {
            return 0;
        }
        if w == 1 || h == 1 {
            return w * h;
        }

        // 2*W + 2*H - 4 corners (to avoid double counting)
        (2 * w) + (2 * h) - 4
    }

    #[inline]
    fn border_contains(&self, pos: (i32, i32)) -> bool {
        if !self.contains(pos) {
            return false;
        }

        pos.0 == self.min.0 ||                  // Left
        pos.0 == self.max.0 - 1 ||              // Right
        pos.1 == self.min.1 ||                  // Top
        pos.1 == self.max.1 - 1 // Bottom
    }

    fn iter_border(&'_ self) -> BoxedShapeIter<'_> {
        // Optimization: Use a dedicated border iterator to avoid allocating a HashSet
        Box::new(RectangleBorderIter::new(self.min, self.max))
    }
}

impl IntoIterator for Rectangle {
    type Item = (i32, i32);
    type IntoIter = RectangleIter;

    fn into_iter(self) -> Self::IntoIter {
        RectangleIter::new(self.min, self.max)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectangle [min(inclusive): ({}, {}), max(exclusive): ({}, {}), size: ({}x{})]",
            self.min.0,
            self.min.1,
            self.max.0,
            self.max.1,
            self.width(),
            self.height()
        )
    }
}

impl From<Rectangle> for BoxedShape {
    fn from(value: Rectangle) -> Self {
        Box::new(value)
    }
}
