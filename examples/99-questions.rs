use ratatui::{
    crossterm::event,
    layout::{Constraint, Layout},
    widgets::{Block, Borders}
};

fn main() {
    let mut terminal = ratatui::init();
    terminal
        .draw(|frame| {
            let layout = Layout::vertical([
                Constraint::Fill(0),
                Constraint::Length(8),
                Constraint::Fill(0)
            ])
            .split(frame.area());
            let text = tui_big_text::BigText::builder()
                .lines(vec!["Any Questions?".into()])
                .centered()
                .build();
            let block = Block::new()
                .borders(Borders::ALL)
                .title_top(
                    "Getting Started With Ratatui"
                );
            frame.render_widget(block, frame.area());
            frame.render_widget(text, layout[1]);
        })
        .unwrap();
    event::read().unwrap();
    ratatui::restore();
}
