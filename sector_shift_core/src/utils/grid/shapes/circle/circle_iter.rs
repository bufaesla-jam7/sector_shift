/// Scanline iterator for a filled circle.
/// Zero allocation.
#[derive(Debug, Clone)]
pub struct CircleIter {
    center: (i32, i32),
    radius_sq: i32,
    current_dy: i32,
    current_dx: i32,
    row_width: i32,
}

impl CircleIter {
    pub fn new(center: (i32, i32), radius: u32) -> Self {
        let r = radius as i32;
        // Start at top row (-r)
        let start_dy = -r;
        // Width at -r is 0 (or 1 point)
        let row_width = 0;

        Self {
            center,
            radius_sq: r * r,
            current_dy: start_dy,
            current_dx: 0,
            row_width,
        }
    }
}

impl Iterator for CircleIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let r = (self.radius_sq as f32).sqrt() as i32;

        // If we exhausted the current row
        if self.current_dx > self.row_width {
            self.current_dy += 1;

            // If we are past the bottom of the circle
            if self.current_dy > r {
                return None;
            }

            // Calculate width for new row: x = sqrt(r^2 - y^2)
            let rem_sq = self.radius_sq - (self.current_dy * self.current_dy);
            if rem_sq < 0 {
                return None; // Should not happen if logic is correct
            }
            self.row_width = (rem_sq as f32).sqrt() as i32;
            self.current_dx = -self.row_width;
        }

        let point = (
            self.center.0 + self.current_dx,
            self.center.1 + self.current_dy,
        );

        self.current_dx += 1;
        Some(point)
    }
}
