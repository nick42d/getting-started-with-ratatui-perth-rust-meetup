#[allow(unreachable_code)]
fn main() {
    let mut terminal = ratatui::init();
    loop {
        let app = ratatui::widgets::Paragraph::new(
            "Hello, World!"
        )
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
