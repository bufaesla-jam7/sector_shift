/// Bresenham's Circle Algorithm Iterator for the border.
/// Buffers 8 symmetric points at a time.
#[derive(Debug, Clone)]
pub struct CircleBorderIter {
    center: (i32, i32),
    x: i32,
    y: i32,
    d: i32,
    buffer: [Option<(i32, i32)>; 8],
    buffer_index: usize,
    done: bool,
}

impl CircleBorderIter {
    pub fn new(center: (i32, i32), radius: u32) -> Self {
        let r = radius as i32;
        // Initial Bresenham state
        let x = 0;
        let y = r;
        let d = 3 - 2 * r;

        let mut iter = Self {
            center,
            x,
            y,
            d,
            buffer: [None; 8],
            buffer_index: 8, // Force refill on first next()
            done: false,
        };

        // Prepare first batch if radius > 0
        if r == 0 {
            iter.buffer[0] = Some(center);
            iter.buffer_index = 0;
            iter.done = true; // One point only
        }

        iter
    }

    fn refill_buffer(&mut self) {
        if self.x > self.y {
            self.done = true;
            return;
        }

        // Generate 8 octant points
        let (cx, cy) = self.center;
        let x = self.x;
        let y = self.y;

        // We use a temporary list and filter duplicates (e.g. if x=0 or x=y)
        let raw_points = [
            (cx + x, cy + y),
            (cx - x, cy + y),
            (cx + x, cy - y),
            (cx - x, cy - y),
            (cx + y, cy + x),
            (cx - y, cy + x),
            (cx + y, cy - x),
            (cx - y, cy - x),
        ];

        // Deduplicate and fill buffer
        // A simple linear scan is fast enough for 8 items
        let mut count = 0;
        self.buffer = [None; 8];

        for &p in &raw_points {
            let mut unique = true;
            for i in 0..count {
                if self.buffer[i] == Some(p) {
                    unique = false;
                    break;
                }
            }
            if unique {
                self.buffer[count] = Some(p);
                count += 1;
            }
        }

        self.buffer_index = 0;

        // Advance Bresenham state
        if self.d < 0 {
            self.d += 4 * self.x + 6;
        } else {
            self.d += 4 * (self.x - self.y) + 10;
            self.y -= 1;
        }
        self.x += 1;
    }
}

impl Iterator for CircleBorderIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // If buffer is empty/exhausted, try refill
        if self.buffer_index >= 8 || self.buffer[self.buffer_index].is_none() {
            if self.done {
                return None;
            }
            self.refill_buffer();

            // If refill didn't produce points (done was set during refill), stop
            self.buffer[self.buffer_index]?;
        }

        let p = self.buffer[self.buffer_index];
        self.buffer_index += 1;
        p
    }
}
