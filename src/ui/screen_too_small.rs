use ratatui::style::{Color, Stylize};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use super::centered_box;

pub fn show(f: &mut Frame) {
    let area = f.area();
    let message = "Terminal must be 30x70!";
    let (w, h) = (message.len() as _, message.lines().count() as _);

    let center = centered_box(w, h, area);

    f.render_widget(Paragraph::new(message).fg(Color::Red), center);
}
