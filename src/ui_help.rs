use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

const ENTRIES: [(&str, &str); 9] = [
    ("Q, Esc", "Quit"),
    ("?", "Toggle Help"),
    ("s, S", "Brush size down/up"),
    ("f, F", "Cycle brush fg next/prev"),
    ("b, B", "Cycle brush bg next/prev"),
    ("y", "Copy canvas to clipboard"),
    ("Y", "Copy canvas as plain text"),
    ("p, P", "Input character from clipboard"),
    ("R", "Reset"),
];

pub fn show(f: &mut ratatui::Frame) {
    let h_pad = 2;
    let help_width = 4 * (h_pad)
        + 1
        + ENTRIES
            .iter()
            .map(|(k, v)| k.len() + v.len())
            .max()
            .unwrap_or(35) as u16;
    let help_height = 4 + ENTRIES.len() as u16;
    let area = f.size();
    let (app_width, app_height) = (area.width, area.height);

    let vert_center = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((app_height - help_height) / 2),
            Constraint::Length(help_height),
            Constraint::Length((app_height - help_height) / 2),
        ])
        .split(area)[1];

    let help_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((app_width - help_width) / 2),
            Constraint::Max(help_width),
            Constraint::Length((app_width - help_width) / 2),
        ])
        .split(vert_center)[1];

    let help_box = Block::default()
        .title(" HELP ")
        .title_style(Style::new().bold().fg(Color::Yellow))
        .title_alignment(Alignment::Center)
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::Yellow));

    let help_box_size = help_box.inner(help_area);

    f.render_widget(Clear, help_area);
    f.render_widget(help_box, help_area);

    let help_box_layout = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(h_pad)
        .constraints([Constraint::Min(1); ENTRIES.len()])
        .split(help_box_size);

    for (i, (key, label)) in ENTRIES.into_iter().enumerate() {
        let row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(help_box_layout[i]);

        f.render_widget(
            Paragraph::new(key)
                .alignment(Alignment::Left)
                .fg(Color::LightYellow),
            row[0],
        );
        f.render_widget(
            Paragraph::new(label)
                .alignment(Alignment::Right)
                .fg(Color::LightYellow),
            row[1],
        );
    }
}
