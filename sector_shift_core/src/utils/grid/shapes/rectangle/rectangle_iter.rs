/// Efficient iterator for full Rectangle area
#[derive(Debug, Clone)]
pub struct RectangleIter {
    current: (i32, i32),
    min: (i32, i32),
    max: (i32, i32),
}

impl RectangleIter {
    pub fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        Self {
            current: min,
            min,
            max,
        }
    }
}

impl Iterator for RectangleIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.1 >= self.max.1 {
            return None;
        }

        let result = self.current;

        // Advance X
        self.current.0 += 1;

        // Wrap to next line if X hits max
        if self.current.0 >= self.max.0 {
            self.current.0 = self.min.0;
            self.current.1 += 1;
        }

        Some(result)
    }
}
