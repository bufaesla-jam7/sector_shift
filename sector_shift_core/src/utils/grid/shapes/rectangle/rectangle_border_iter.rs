/// Efficient iterator for Rectangle perimeter
#[derive(Debug, Clone)]
pub struct RectangleBorderIter {
    current: (i32, i32),
    min: (i32, i32),
    max: (i32, i32),
    phase: u8, // 0=Top, 1=Right, 2=Bottom, 3=Left
}

impl RectangleBorderIter {
    pub fn new(min: (i32, i32), max: (i32, i32)) -> Self {
        Self {
            current: min,
            min,
            max,
            phase: 0,
        }
    }
}

impl Iterator for RectangleBorderIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // Handle degenerate cases (lines or points)
        let w = self.max.0 - self.min.0;
        let h = self.max.1 - self.min.1;
        if w <= 0 || h <= 0 {
            return None;
        }

        // If 1D/Point, just dump the area once and stop
        if w == 1 || h == 1 {
            if self.current.0 < self.max.0 && self.current.1 < self.max.1 {
                let res = self.current;
                // Simple linear advance
                self.current.0 += 1;
                if self.current.0 >= self.max.0 {
                    self.current.0 = self.min.0;
                    self.current.1 += 1;
                }
                return Some(res);
            }
            return None;
        }

        loop {
            match self.phase {
                0 => {
                    // Top Edge: (min.x ... max.x-1, min.y)
                    if self.current.0 < self.max.0 - 1 {
                        let res = (self.current.0, self.min.1);
                        self.current.0 += 1;
                        return Some(res);
                    }
                    self.phase = 1;
                    self.current.1 = self.min.1; // Setup for Right
                },
                1 => {
                    // Right Edge: (max.x-1, min.y ... max.y-1)
                    if self.current.1 < self.max.1 - 1 {
                        let res = (self.max.0 - 1, self.current.1);
                        self.current.1 += 1;
                        return Some(res);
                    }
                    self.phase = 2;
                    self.current.0 = self.max.0 - 1; // Setup for Bottom
                },
                2 => {
                    // Bottom Edge: (max.x-1 ... min.x+1, max.y-1)
                    if self.current.0 > self.min.0 {
                        let res = (self.current.0, self.max.1 - 1);
                        self.current.0 -= 1;
                        return Some(res);
                    }
                    self.phase = 3;
                    self.current.1 = self.max.1 - 1; // Setup for Left
                },
                3 => {
                    // Left Edge: (min.x, max.y-1 ... min.y+1)
                    if self.current.1 > self.min.1 {
                        let res = (self.min.0, self.current.1);
                        self.current.1 -= 1;
                        return Some(res);
                    }
                    self.phase = 4; // Done
                    return None;
                },
                _ => return None,
            }
        }
    }
}
