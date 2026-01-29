use crate::utils::grid::shapes::Shape;

/// Boxed shape
pub type BoxedShape = Box<dyn Shape + Send + Sync>;

/// Boxed shape iterator
pub type BoxedShapeIter<'a> = Box<dyn Iterator<Item = (i32, i32)> + 'a>;
