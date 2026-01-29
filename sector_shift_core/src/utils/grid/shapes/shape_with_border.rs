use crate::utils::grid::shapes::{BoxedShapeIter, Shape};

/// A trait for dealing with 2D shapes with a border
pub trait ShapeWithBorder: Shape {
    /// returns the number of points on the border
    fn border_count(&self) -> usize;

    /// returns `true` if the point is inside the shape
    fn border_contains(&self, position: (i32, i32)) -> bool;

    /// returns an iterator over all of the points
    fn iter_border(&'_ self) -> BoxedShapeIter<'_>;
}
