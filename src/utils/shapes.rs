use crate::app::App;

pub struct DrawShape {
    x: i16,
    y: i16,
    size: i16,
}

impl DrawShape {
    #[must_use]
    pub const fn new(x: u16, y: u16, size: u16) -> Self {
        let (x, y, size) = (x as i16, y as i16, size as i16);
        Self { x, y, size }
    }

    pub fn point(&self, app: &mut App) {
        app.draw(self.x, self.y);
    }

    const fn get_brush_rect(&self) -> (i16, i16, i16, i16) {
        // Calculate brush offset
        let brush_offset = (self.size - 1) / 2;
        // Left and bottom
        let left = self.x - brush_offset;
        let bottom = self.y - brush_offset;
        // Right and top
        let right = left + self.size;
        let top = bottom + self.size;

        (left, right, bottom, top)
    }

    pub fn eraser(&self, app: &mut App) {
        let (left, right, bottom, top) = self.get_brush_rect();

        for x in left..right {
            for y in bottom..top {
                app.erase(x, y);
            }
        }
    }

    pub fn square(&self, app: &mut App) {
        let (left, right, bottom, top) = self.get_brush_rect();

        // Loop through the brush coordinates and write the values
        for x in left..right {
            for y in bottom..top {
                app.draw(x, y);
            }
        }
    }

    pub fn rect(&self, app: &mut App) {
        let (left, right, bottom, top) = self.get_brush_rect();

        for y in bottom..top {
            match y {
                y if y == bottom || y == top - 1 => (left..right).for_each(|x| app.draw(x, y)),
                y => [left, right - 1].into_iter().for_each(|x| app.draw(x, y)),
            }
        }
    }

    pub fn circle(&self, app: &mut App) {
        let mut x = self.size;
        let mut y = 0;
        let mut p = 1 - x;

        // if self.size > 0 {
        app.draw(self.x + x, self.y);
        app.draw(self.x - x, self.y);
        app.draw(self.x, self.y + x);
        app.draw(self.x, self.y - x);
        // }

        while x > y {
            y += 1;
            if p <= 0 {
                p = p + 2 * y + 1;
            } else {
                x -= 1;
                p = p + 2 * y - 2 * x + 1;
            }

            if x < y {
                break;
            }

            // Draw the points at the circumference
            app.draw(self.x + x, self.y + y);
            app.draw(self.x - x, self.y + y);
            app.draw(self.x + x, self.y - y);
            app.draw(self.x - x, self.y - y);

            if x != y {
                app.draw(self.x + y, self.y + x);
                app.draw(self.x - y, self.y + x);
                app.draw(self.x + y, self.y - x);
                app.draw(self.x - y, self.y - x);
            }
        }
    }

    pub fn disk(&self, app: &mut App) {
        let (cx, cy, radius) = (self.x, self.y, self.size);

        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= radius * radius {
                    app.draw(cx + x, cy + y);
                }
            }
        }
    }

    pub fn plus(&self, app: &mut App) {
        app.draw(self.x, self.y);

        if self.size == 1 {
            return;
        }

        for s in [-1, 1] {
            for i in 1..=self.size {
                app.draw(s * i + self.x, self.y);
                app.draw(self.x, s * i + self.y);
            }
        }
    }

    pub fn horiz(&self, app: &mut App) {
        app.draw(self.x, self.y);

        if self.size == 1 {
            return;
        }

        for s in [-1, 1] {
            for i in 1..=self.size {
                app.draw(s * i + self.x, self.y);
            }
        }
    }

    pub fn vert(&self, app: &mut App) {
        app.draw(self.x, self.y);

        if self.size == 1 {
            return;
        }

        for s in [-1, 1] {
            for i in 1..=self.size {
                app.draw(self.x, s * i + self.y);
            }
        }
    }
}
