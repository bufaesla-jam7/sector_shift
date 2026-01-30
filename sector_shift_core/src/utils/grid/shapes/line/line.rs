use std::fmt::Display;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::grid::shapes::{BoxedShape, BoxedShapeIter, Shape, line::LineIter};

/// A 2D line segment defined by a start and end point.
#[derive(Serialize, Deserialize, Reflect, Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Line {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

impl Line {
    /// Creates a new line segment.
    #[inline]
    pub const fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        Self { start, end }
    }

    /// Returns the Chebyshev distance (max of dx, dy).
    /// This represents the number of steps to travel from start to end.
    #[inline]
    pub fn length(&self) -> u32 {
        let dx = (self.end.0 - self.start.0).abs();
        let dy = (self.end.1 - self.start.1).abs();
        dx.max(dy) as u32
    }
}

impl Shape for Line {
    #[inline]
    fn count(&self) -> usize {
        // Line points = steps + 1 (start point)
        (self.length() as usize) + 1
    }

    #[inline]
    fn contains(&self, position: (i32, i32)) -> bool {
        // Fast Bounding Box Check
        let min_x = self.start.0.min(self.end.0);
        let max_x = self.start.0.max(self.end.0);
        let min_y = self.start.1.min(self.end.1);
        let max_y = self.start.1.max(self.end.1);

        if position.0 < min_x || position.0 > max_x || position.1 < min_y || position.1 > max_y {
            return false;
        }

        // Exact check using the iterator (Bresenham path is specific)
        // Since lines are typically short in grids, O(N) is acceptable and avoids
        // complex integer math edge cases with "ideal" lines vs "pixel" lines.
        self.iter().any(|p| p == position)
    }

    #[inline]
    fn iter(&'_ self) -> BoxedShapeIter<'_> {
        Box::new(LineIter::new(self.start, self.end))
    }
}

impl IntoIterator for Line {
    type Item = (i32, i32);
    type IntoIter = LineIter;

    fn into_iter(self) -> Self::IntoIter {
        LineIter::new(self.start, self.end)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line [start: ({}, {}), end: ({}, {}), length: {}]",
            self.start.0,
            self.start.1,
            self.end.0,
            self.end.1,
            self.length()
        )
    }
}

impl From<Line> for BoxedShape {
    fn from(value: Line) -> Self {
        Box::new(value)
    }
}
