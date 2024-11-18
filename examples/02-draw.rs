use ratatui::{
    style::{Color, Style},
    widgets::Paragraph
};

fn main() {
    let mut terminal = ratatui::init();
    let app = Paragraph::new("Hello, World!").style(
        Style::new().fg(Color::Red).bg(Color::White)
    );
    terminal
        .draw(|frame| {
            frame.render_widget(app, frame.area())
        })
        .unwrap();
    std::thread::sleep(std::time::Duration::new(2, 0));
    ratatui::restore();
}
