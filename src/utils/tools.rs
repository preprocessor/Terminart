use strum::{EnumIter, IntoEnumIterator};

use crate::app::App;

use super::shapes::DrawShape;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumIter)]
pub enum Tool {
    Eraser,
    #[default]
    Square,
    Box,
    Disk,
    Circle,
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
        Self::iter().collect()
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

    pub fn draw(&self, x: u16, y: u16, size: u16, app: &mut App) {
        let draw = DrawShape::new(x, y, size);

        match self {
            Self::Eraser => draw.eraser(app),
            Self::Square => draw.square(app),
            Self::Box => draw.rect(app),
            Self::Disk => draw.disk(app),
            Self::Circle => draw.circle(app),
            Self::Point => draw.point(app),
            Self::Plus => draw.plus(app),
            Self::Vertical => draw.vert(app),
            Self::Horizontal => draw.horiz(app),
        }
    }
}
