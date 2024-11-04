use ratatui::{
    crossterm::{self, event::*},
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{ListState, Paragraph}
};

#[allow(unreachable_code, clippy::single_match)]
fn main() {
    let mut terminal = ratatui::init();
    let mut state = ListState::default();
    state.select(Some(0));
    while state.selected().is_some_and(|s| s < 10) {
        let list =
            ["ratatui", "hello", "ratatouille", "world"];
        let title = ratatui::widgets::List::new(
            list.iter()
                .cycle()
                .take(20)
                .filter(|s| !s.contains("rat"))
                .map(|s| s.to_uppercase())
        )
        .highlight_symbol(">>")
        .white()
        .on_red();
        let body = Paragraph::new(format!(
            "Iteration x, press (+) to increase"
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
            }) => state.select_next(),
            _ => ()
        }
    }
    ratatui::restore();
}
