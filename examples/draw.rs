fn main() {
    let mut terminal = ratatui::init();
    let app = ratatui::widgets::Paragraph::new(
        "Hello, World!"
    )
    .style(
        ratatui::style::Style::new()
            .fg(ratatui::style::Color::Blue)
            .bg(ratatui::style::Color::Green)
    );
    terminal
        .draw(|frame| {
            frame.render_widget(app, frame.area())
        })
        .unwrap();
    ratatui::restore();
}
