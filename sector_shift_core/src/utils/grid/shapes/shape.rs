use crate::utils::grid::shapes::BoxedShapeIter;

/// A trait for dealing with 2D shapes
pub trait Shape: Send + Sync {
    /// returns the number of points in the shape
    fn count(&self) -> usize {
        self.iter().count()
    }

    /// returns `true` if the point is inside the shape
    fn contains(&self, position: (i32, i32)) -> bool;

    /// returns an iterator over all of the points
    fn iter(&'_ self) -> BoxedShapeIter<'_>;
}
