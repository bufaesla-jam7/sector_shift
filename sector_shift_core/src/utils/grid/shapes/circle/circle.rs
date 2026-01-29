use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::grid::shapes::{
    BoxedShapeIter, Shape, ShapeWithBorder,
    circle::{CircleBorderIter, CircleIter},
    line::Line,
};

/// A 2D circle.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Circle {
    pub center: (i32, i32),
    pub radius: u32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: (0, 0),
            radius: 1,
        }
    }
}

impl Circle {
    /// Creates a new circle.
    #[inline]
    pub const fn new(center: (i32, i32), radius: u32) -> Self {
        Self { center, radius }
    }

    /// Get the left point of the circle (min x).
    #[inline]
    pub fn left(&self) -> (i32, i32) {
        (self.center.0 - self.radius as i32, self.center.1)
    }

    /// Get the right point of the circle (max x).
    #[inline]
    pub fn right(&self) -> (i32, i32) {
        (self.center.0 + self.radius as i32, self.center.1)
    }

    /// Get the top point of the circle (max y).
    #[inline]
    pub fn top(&self) -> (i32, i32) {
        (self.center.0, self.center.1 + self.radius as i32)
    }

    /// Get the bottom point of the circle (min y).
    #[inline]
    pub fn bottom(&self) -> (i32, i32) {
        (self.center.0, self.center.1 - self.radius as i32)
    }

    /// Return a line from the left to the right of the circle.
    #[inline]
    pub fn as_horizontal_line(&self) -> Line {
        Line::new(self.left(), self.right())
    }

    /// Return a line from the top to the bottom of the circle.
    #[inline]
    pub fn as_vertical_line(&self) -> Line {
        Line::new(self.bottom(), self.top())
    }
}

impl Shape for Circle {
    #[inline]
    fn count(&self) -> usize {
        // Exact count via iteration is safest for integer grid circles
        self.iter().count()
    }

    #[inline]
    fn contains(&self, position: (i32, i32)) -> bool {
        let dx = position.0 - self.center.0;
        let dy = position.1 - self.center.1;
        let dist_sq = dx * dx + dy * dy;
        dist_sq <= (self.radius * self.radius) as i32
    }

    #[inline]
    fn iter(&'_ self) -> BoxedShapeIter<'_> {
        Box::new(CircleIter::new(self.center, self.radius))
    }
}

impl ShapeWithBorder for Circle {
    #[inline]
    fn border_count(&self) -> usize {
        self.iter_border().count()
    }

    #[inline]
    fn border_contains(&self, position: (i32, i32)) -> bool {
        // Fast fail check
        if !self.contains(position) {
            return false;
        }
        // Exact check: iterate border.
        // Since perimeter is O(Radius), this is fast enough (e.g., R=100 -> ~628 checks).
        self.iter_border().any(|p| p == position)
    }

    #[inline]
    fn iter_border(&'_ self) -> BoxedShapeIter<'_> {
        Box::new(CircleBorderIter::new(self.center, self.radius))
    }
}

impl IntoIterator for Circle {
    type Item = (i32, i32);
    type IntoIter = CircleIter;

    fn into_iter(self) -> Self::IntoIter {
        CircleIter::new(self.center, self.radius)
    }
}
