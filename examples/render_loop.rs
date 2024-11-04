use ratatui::{
    style::{Color, Style},
    widgets::Paragraph
};
use std::time::Duration;

#[allow(unreachable_code)]
fn main() {
    let mut terminal = ratatui::init();
    let mut counter = 0;
    while counter <= 10 {
        let app = Paragraph::new(format!(
            "Hello, World!\nIteration {counter}"
        ))
        .style(
            Style::new().fg(Color::Red).bg(Color::White)
        );
        terminal
            .draw(|f| f.render_widget(app, f.area()))
            .unwrap();
        counter += 1;
        std::thread::sleep(Duration::from_millis(300));
    }
    ratatui::restore();
}
