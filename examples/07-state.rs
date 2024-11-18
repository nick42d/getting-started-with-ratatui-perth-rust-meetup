use ratatui::{
    crossterm::{self, event::*},
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{ListState, Paragraph}
};

#[allow(unreachable_code, clippy::single_match)]
fn main() {
    let mut terminal = ratatui::init();
    let mut state = ListState::default();
    let mut counter = 0;
    while counter <= 15 {
        state.select(Some(counter));
        let list =
            ["ratatui", "hello", "ratatouille", "world"];
        let title = ratatui::widgets::List::new(
            list.iter()
                .cycle()
                .take(20)
                .map(|s| s.to_uppercase())
        )
        .highlight_symbol(">>")
        .white()
        .on_red();
        let body =
            Paragraph::new("press (+) to increase")
                .red()
                .on_white();
        terminal
            .draw(|f| {
                let layout = Layout::vertical([
                    Constraint::Percentage(25),
                    Constraint::Percentage(75)
                ])
                .split(f.area());
                f.render_stateful_widget(
                    title, layout[0], &mut state
                );
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
