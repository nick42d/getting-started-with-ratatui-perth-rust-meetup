use std::time::SystemTime;

#[allow(unreachable_code)]
fn main() {
    let mut terminal = ratatui::init();
    loop {
        let time = SystemTime::now();
        let app =
            ratatui::widgets::Paragraph::new(format!(
                "Hello, World!\nSystem time is: {:?}",
                time
            ))
            .style(
                ratatui::style::Style::new()
                    .fg(ratatui::style::Color::Red)
                    .bg(ratatui::style::Color::White)
            );
        terminal
            .draw(|frame| {
                frame.render_widget(app, frame.area())
            })
            .unwrap();
    }
    ratatui::restore();
}
