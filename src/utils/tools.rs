use crate::app::App;

use super::layer::LayerData;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tool {
    Eraser,
    Square,
    Box,
    Disk,
    Circle,
    #[default]
    Point,
    Plus,
    Vertical,
    Horizontal,
}

impl Tool {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Eraser,
            Self::Square,
            Self::Box,
            Self::Disk,
            Self::Circle,
            Self::Point,
            Self::Plus,
            Self::Vertical,
            Self::Horizontal,
        ]
    }

    pub fn char(&self) -> String {
        match self {
            Self::Eraser => '×',
            Self::Square => '■',
            Self::Box => '□',
            Self::Disk => '●',
            Self::Circle => '○',
            Self::Point => '∙',
            Self::Plus => '🞣',
            Self::Vertical => '|',
            Self::Horizontal => '─',
        }
        .to_string()
    }

    pub fn draw(&self, x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
        match self {
            Tool::Eraser => eraser_tool(x, y, size, app),
            Tool::Square => square_tool(x, y, size, app),
            Tool::Box => box_tool(x, y, size, app),
            Tool::Disk => todo!("Disk (filled Circle)"),
            Tool::Circle => circle_tool(x, y, size, app),
            Tool::Point => {
                let mut old_cell = LayerData::new();
                old_cell.insert((x, y), app.draw(x, y));
                old_cell
            }
            Tool::Plus => plus(x, y, size, app),
            Tool::Vertical => vert(x, y, size, app),
            Tool::Horizontal => horiz(x, y, size, app),
        }
    }
}

pub fn plus(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();

    old_cells.insert((x, y), app.draw(x, y));

    if size == 1 {
        return old_cells;
    }

    for s in [-1, 1] as [i16; 2] {
        for i in 1..=size as i16 {
            let x_arm = s * i + x as i16;
            let y_arm = s * i + y as i16;
            if x_arm >= 0 {
                let x_arm = x_arm as u16;
                old_cells.insert((x_arm, y), app.draw(x_arm, y));
            }
            if y_arm >= 0 {
                let y_arm = y_arm as u16;
                old_cells.insert((x, y_arm), app.draw(x, y_arm));
            }
        }
    }

    old_cells
}

pub fn horiz(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();

    old_cells.insert((x, y), app.draw(x, y));

    if size == 1 {
        return old_cells;
    }

    for s in [-1, 1] as [i16; 2] {
        for i in 1..=size as i16 {
            let x_arm = s * i + x as i16;
            if x_arm >= 0 {
                let x_arm = x_arm as u16;
                old_cells.insert((x_arm, y), app.draw(x_arm, y));
            }
        }
    }

    old_cells
}

pub fn vert(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();

    old_cells.insert((x, y), app.draw(x, y));

    if size == 1 {
        return old_cells;
    }

    for s in [-1, 1] as [i16; 2] {
        for i in 1..=size as i16 {
            let y_arm = s * i + y as i16;
            if y_arm >= 0 {
                let y_arm = y_arm as u16;
                old_cells.insert((x, y_arm), app.draw(x, y_arm));
            }
        }
    }

    old_cells
}

// fn disk_tool(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
//     let mut old_cells = LayerData::new();
//     let (x, y) = (x as i16, y as i16);
//
//     let mut x_r = size as i16;
//     let mut y_r = 0;
//     let mut p = 1 - x_r;
//
//     // if self.size > 0 {
//     app.draw(x + x_r, y);
//     app.draw(x - x_r, y);
//     app.draw(x, y + x_r);
//     app.draw(x, y - x_r);
//     // }
//
//     while x_r > y_r {
//         y_r += 1;
//         if p <= 0 {
//             p = p + 2 * y_r + 1;
//         } else {
//             x_r -= 1;
//             p = p + 2 * y_r - 2 * x_r + 1;
//         }
//
//         if x_r < y_r {
//             break;
//         }
//
//         // Draw the points at the circumference
//         app.draw(x + x_r, y + y_r);
//         app.draw(x - x_r, y + y_r);
//         app.draw(x + x_r, y - y_r);
//         app.draw(x - x_r, y - y_r);
//
//         if x_r != y_r {
//             app.draw(x + y_r, y + x_r);
//             app.draw(x - y_r, y + x_r);
//             app.draw(x + y_r, y - x_r);
//             app.draw(x - y_r, y - x_r);
//         }
//     }
//     old_cells
// }

fn circle_tool(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();

    let (cx, cy, radius) = (x as i16, y as i16, size as i16);

    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                let (fx, fy) = (cx + x, cy + y);
                if fx < 0 || fy < 0 {
                    continue;
                }
                let (fx, fy) = (fx as u16, fy as u16);
                old_cells.insert((fx, fy), app.draw(fx, fy));
            }
        }
    }

    old_cells
}

fn box_tool(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();
    let (left, right, bottom, top) = get_brush_rect_i16(x, y, size);
    for y in bottom..top {
        if y < 0 {
            continue;
        };

        if y == bottom || y == top {
            for x in left..right {
                if x < 0 {
                    continue;
                };
                let (x, y) = (x as u16, y as u16);

                if old_cells.contains_key(&(x, y)) {
                    continue;
                }
                old_cells.insert((x, y), app.draw(x, y));
            }
        } else {
            for x in [left, right - 1] {
                if x < 0 {
                    continue;
                };
                let (x, y) = (x as u16, y as u16);
                // old_cells.push(app.draw2(x, y))
                if old_cells.contains_key(&(x, y)) {
                    continue;
                }
                old_cells.insert((x, y), app.draw(x, y));
            }
        }
    }
    old_cells
}

fn square_tool(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();
    let (left, right, bottom, top) = get_brush_rect_i16(x, y, size);
    // Loop through the brush coordinates and write the values
    for x in left..right {
        for y in bottom..top {
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as u16, y as u16);

            if old_cells.contains_key(&(x, y)) {
                continue;
            }
            old_cells.insert((x, y), app.draw(x, y));
        }
    }
    old_cells
}

fn eraser_tool(x: u16, y: u16, size: u16, app: &mut App) -> LayerData {
    let mut old_cells = LayerData::new();
    let (left, right, bottom, top) = get_brush_rect_u16(x, y, size);

    for x in left..right {
        for y in bottom..top {
            if old_cells.contains_key(&(x, y)) {
                continue;
            }
            old_cells.insert((x, y), app.erase(x, y));
        }
    }

    old_cells
}

fn get_brush_rect_i16(x: u16, y: u16, size: u16) -> (i16, i16, i16, i16) {
    // Allow negatives
    let (x, y, size) = (x as i16, y as i16, size as i16);
    // Calculate brush offset
    let brush_offset = (size - 1) / 2;
    // Left and bottom
    let left = x - brush_offset;
    let bottom = y - brush_offset;
    // Right and top
    let right = left + size;
    let top = bottom + size;

    (left, right, bottom, top)
}

fn get_brush_rect_u16(x: u16, y: u16, size: u16) -> (u16, u16, u16, u16) {
    // Calculate brush offset
    let brush_offset = size.saturating_sub(1) / 2;
    // Left and bottom
    let left = x.saturating_sub(brush_offset);
    let bottom = y.saturating_sub(brush_offset);
    // Right and top
    let right = x + brush_offset;
    let top = y + brush_offset;

    (left, right, bottom, top)
}
