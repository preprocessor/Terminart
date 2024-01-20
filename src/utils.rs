use ahash::AHashMap;
use ratatui::style::{Color, Style};
use strum::{EnumIter, IntoEnumIterator};

pub type Page = AHashMap<(u16, u16), Cell>;

// Cell {
#[derive(Clone, Copy, Debug, Default)]
pub struct Cell {
    pub fg: Color,
    pub bg: Color,
    pub char: char,
}

impl Cell {
    #[rustfmt::skip]    pub fn char(&self) -> String { self.char.to_string() }

    pub const fn style(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    /// For use with the undo history, for removing a cell use app.erase
    pub const fn empty() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            char: ' ',
        }
    }
}
// }

// Brush: {
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, EnumIter)]
pub enum Tool {
    Eraser,
    #[default]
    Square,
    Box,
    Disk,
    Circle,
    Crosshair,
    Plus,
    Vertical,
    Horizontal,
}

impl Tool {
    pub fn char(&self) -> String {
        match self {
            Self::Eraser => 'x',
            Self::Square => '■',
            Self::Box => '□',
            Self::Disk => '●',
            Self::Circle => '○',
            Self::Crosshair => '+',
            Self::Plus => '🞣',
            Self::Vertical => '|',
            Self::Horizontal => '-',
        }
        .to_string()
    }

    #[rustfmt::skip]    pub fn name(&self) -> String { format!("{:?}", self) }
    #[rustfmt::skip]    pub fn all() -> Vec<Self> { Self::iter().collect() }
}

#[derive(Clone, Copy, Debug)]
pub struct Brush {
    pub size: u16,
    pub fg: Color,
    pub bg: Color,
    pub char: char,
    pub tool: Tool,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            size: 1,
            fg: Color::Black,
            bg: Color::White,
            char: '░',
            tool: Tool::default(),
        }
    }
}

impl Brush {
    pub const fn style(&self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    pub const fn as_cell(&self) -> Cell {
        Cell {
            fg: self.fg,
            bg: self.bg,
            char: self.char,
        }
    }

    #[rustfmt::skip]    pub fn char(&self) -> String { self.char.to_string() }
}
// }

// Palette {
#[derive(Clone, Debug)]
pub struct Palette {
    colors: Vec<Color>,
    fg: usize,
    bg: usize,
}

impl Default for Palette {
    fn default() -> Self {
        let colors = vec![
            Color::Black,        //  0
            Color::Red,          //  1
            Color::Yellow,       //  2
            Color::Green,        //  3
            Color::Blue,         //  4
            Color::Magenta,      //  5
            Color::Cyan,         //  6
            Color::Gray,         //  7
            Color::DarkGray,     //  8
            Color::LightRed,     //  9
            Color::LightYellow,  // 10
            Color::LightGreen,   // 11
            Color::LightBlue,    // 12
            Color::LightMagenta, // 13
            Color::LightCyan,    // 14
            Color::White,        // 15
        ];
        Self {
            colors,
            fg: 0,
            bg: 15,
        }
    }
}

impl Palette {
    pub fn colors(&self) -> Vec<Color> {
        self.colors.to_vec()
    }

    fn next(&self, i: usize) -> usize {
        (i + 1) % self.colors.len()
    }

    fn prev(&self, i: usize) -> usize {
        i.checked_sub(1).unwrap_or(self.colors.len() - 1)
    }

    pub fn fg_next(&mut self) -> Color {
        self.fg = self.next(self.fg);
        self.colors[self.fg]
    }

    pub fn fg_prev(&mut self) -> Color {
        self.fg = self.prev(self.fg);
        self.colors[self.fg]
    }

    pub fn bg_next(&mut self) -> Color {
        self.bg = self.next(self.bg);
        self.colors[self.bg]
    }

    pub fn bg_prev(&mut self) -> Color {
        self.bg = self.prev(self.bg);
        self.colors[self.bg]
    }
}
// }

#[derive(Debug, Default)]
pub struct UndoHistory {
    pub past: Vec<Page>,
    pub future: Vec<Page>,
}

impl UndoHistory {
    pub fn try_add_page(&mut self) {
        match self.past.last() {
            None => self.past.push(Page::default()),
            Some(page) if !page.is_empty() => self.past.push(Page::default()),
            _ => {}
        }
    }

    pub fn add_undo(&mut self, x: u16, y: u16, cell: Cell) {
        if self.past.is_empty() {
            self.past.push(Page::default());
        }

        if let Some(page) = self.past.last_mut() {
            let value = if page.contains_key(&(x, y)) {
                // This is to prevent a line that, in a single drag action,
                // intersects itself; from overwriting the original values
                // in the undo history event
                Cell::empty()
            } else {
                cell
            };

            page.insert((x, y), value);
        }
    }

    pub fn add_redo(&mut self, x: u16, y: u16, value: Cell) {
        if self.future.is_empty() {
            self.future.push(Page::default());
        }

        #[allow(clippy::unwrap_used)]
        self.future.last_mut().unwrap().insert((x, y), value);
    }

    pub fn undo(&mut self) -> Option<Page> {
        let undo = self.past.pop()?;
        self.future.push(undo.clone());

        Some(undo)
    }

    pub fn redo(&mut self) -> Option<Page> {
        let redo = self.future.pop()?;
        self.past.push(redo.clone());

        Some(redo)
    }

    pub fn forget_redo(&mut self) {
        self.future.clear();
    }
}

// ClickAction and related {
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClickAction {
    None,
    Draw,
    Prev(Increment),
    Next(Increment),
    Set(SetValue),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Increment {
    CharPicker,
    BrushSize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetValue {
    Tool(Tool),
    Color(Color),
    Char(char), // 🦎🔥
}
// }
