/// A self-contained Bresenham Line Iterator.
/// Iterates from `start` to `end` inclusive.
#[derive(Debug, Clone)]
pub struct LineIter {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    dx: i32,
    dy: i32,
    sx: i32,
    sy: i32,
    err: i32,
    done: bool,
}

impl LineIter {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let dx = (end.0 - start.0).abs();
        let dy = (end.1 - start.1).abs();
        let sx = if start.0 < end.0 { 1 } else { -1 };
        let sy = if start.1 < end.1 { 1 } else { -1 };
        let err = dx - dy;

        Self {
            x0: start.0,
            y0: start.1,
            x1: end.0,
            y1: end.1,
            dx,
            dy,
            sx,
            sy,
            err,
            done: false,
        }
    }
}

impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let point = (self.x0, self.y0);

        if self.x0 == self.x1 && self.y0 == self.y1 {
            self.done = true;
            return Some(point);
        }

        let e2 = 2 * self.err;
        if e2 > -self.dy {
            self.err -= self.dy;
            self.x0 += self.sx;
        }
        if e2 < self.dx {
            self.err += self.dx;
            self.y0 += self.sy;
        }

        Some(point)
    }
}
