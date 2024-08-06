use ratatui::layout::Alignment;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph};

use super::{centered_box, YELLOW};

const HELP_TEXT: &str = "
  Q, Esc - Quit
L-Button - (Canvas) Draw with current brush
M-Button - (Canvas) Paste into canvas at mouse cursor
L-Button - (Palette) Set foreground color
R-Button - (Palette) Set background color
M-Button - (Palette) Unset selected color (transparent)
    s, S - Brush size
    f, F - Cycle brush fg
    b, B - Cycle brush bg
       y - Copy canvas to clipboard with ANSI codes
       Y - Copy canvas to clipboard as plain text
    p, P - Input first character from clipboard as brush
Ctrl + S - Save Canvas (flat ANSI)
Ctrl + E - Export Canvas (layers, palette, and brush)
       R - Reset (Will delete layers)
       ? - Toggle Help
";

pub fn show(f: &mut ratatui::Frame) {
    let help_width = 6 + HELP_TEXT.lines().skip(1).fold(0, |a, b| a.max(b.len())) as u16;
    let help_height = 4 + HELP_TEXT.lines().skip(1).count() as u16;

    let help_area = centered_box(help_width, help_height, f.size());

    let help_box = Block::default()
        .title(" HELP ")
        .title_style(Style::new().bold().fg(YELLOW))
        .title_alignment(Alignment::Center)
        .padding(Padding::new(2, 2, 1, 1))
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::Yellow));

    let help_box_size = help_box.inner(help_area);

    f.render_widget(Clear, help_area);
    f.render_widget(help_box, help_area);

    let lines: Vec<_> = HELP_TEXT.lines().skip(1).map(Line::from).collect();

    f.render_widget(Paragraph::new(lines).fg(YELLOW), help_box_size);
}
