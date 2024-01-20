use ahash::AHashMap;

pub type CharMap = AHashMap<(u16, u16), char>;

#[derive(Clone, Debug)]
pub struct CharPicker {
    pub chars: CharMap,
    pub selected: (u16, u16),
    pub page: u16,
}

impl Default for CharPicker {
    fn default() -> Self {
        let mut map = CharMap::default();

        for (y, tens) in (2..=7).enumerate() {
            let tens = tens << 4; // Shift to the right
            for (x, ones) in (0..=0xF).enumerate() {
                let hex = tens + ones;
                // skip over DEL escape code
                if hex == 0x7f {
                    continue;
                }

                if let Some(ch) = std::char::from_u32(hex) {
                    map.insert((x as u16, y as u16), ch);
                }
            }
        }

        // Lowest byte for box drawing chars
        let lines_base = 0x2500;

        for (y, tens) in (0..=9).rev().enumerate() {
            let tens = tens << 4; // Shift to the right
            for (x, ones) in (0..=0xF).enumerate() {
                let hex = lines_base + tens + ones;
                if let Some(ch) = std::char::from_u32(hex) {
                    map.insert((x as u16, 6 + y as u16), ch);
                }
            }
        }

        Self {
            chars: map,
            selected: (0, 0),
            page: 0,
        }
    }
}

impl CharPicker {
    pub const fn rows(&self) -> u16 {
        7
    }

    fn get_page(&self, number: u16) -> Vec<char> {
        // A page is 2 "rows" from the 16x16 hashmap
        let row = number * 2;

        let mut out = Vec::with_capacity(32);

        for sub in 0..=1 {
            for x in 0..=0xF {
                if let Some(&ch) = self.chars.get(&(x, row + sub)) {
                    out.push(ch);
                }
            }
        }

        if out.is_empty() {
            self.get_page(number - 1)
        } else {
            out
        }
    }

    pub fn page(&self) -> Vec<char> {
        self.get_page(self.page)
    }

    pub fn next(&mut self) {
        let next = self.page + 1;
        self.page = if next > self.rows() { 0 } else { next }
    }

    pub fn prev(&mut self) {
        self.page = self.page.checked_sub(1).unwrap_or_else(|| self.rows());
    }
}
