use ratatui::{
    crossterm::{self, event::*},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::Paragraph
};

#[allow(unreachable_code, clippy::single_match)]
fn main() {
    let mut terminal = ratatui::init();
    let mut counter = 0;
    while counter <= 10 {
        let title = Paragraph::new("Hello, World!")
            .white()
            .on_red();
        let body = Paragraph::new(format!(
            "Iteration {counter}, press (+) to increase"
        ))
        .red()
        .on_white();
        terminal
            .draw(|f| {
                let layout = Layout::vertical([
                    Constraint::Percentage(25),
                    Constraint::Percentage(75)
                ])
                .split(f.area());
                f.render_widget(title, layout[0]);
                f.render_widget(body, layout[1]);
            })
            .unwrap();
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(KeyEvent {
                code: KeyCode::Char('+'),
                ..
            }) => counter += 1,
            _ => ()
        }
    }
    ratatui::restore();
}
