use ratatui::{
    crossterm::{
        self,
        event::{Event, KeyCode, KeyEvent}
    },
    style::{Color, Style},
    widgets::Paragraph
};

#[allow(unreachable_code, clippy::single_match)]
fn main() {
    let mut terminal = ratatui::init();
    let mut counter = 0;
    while counter <= 10 {
        let app = Paragraph::new(format!(
            "Hello, World!
Iteration {counter}, press (+) to increase"
        ))
        .style(
            Style::new().fg(Color::Red).bg(Color::White)
        );
        terminal
            .draw(|f| f.render_widget(app, f.area()))
            .unwrap();
        match crossterm::event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('+'),
                ..
            }) => counter += 1,
            _ => ()
        }
    }
    ratatui::restore();
}
