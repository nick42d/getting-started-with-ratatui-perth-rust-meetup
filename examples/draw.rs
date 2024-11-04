fn main() {
    let mut terminal = ratatui::init();
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
    std::thread::sleep(std::time::Duration::new(2, 0));
    ratatui::restore();
}
