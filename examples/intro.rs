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
                Constraint::Length(20),
                Constraint::Fill(0)
            ])
            .split(frame.area());
            let title = tui_big_text::BigText::builder()
                .lines(vec![
                    "Getting Started With".into(),
                    "Ratatui".into(),
                ])
                .centered()
                .build();
            let subtitle =
                tui_big_text::BigText::builder()
                    .lines(vec![
                        "The Rust TUI Library".into()
                    ])
                    .centered()
                    .pixel_size(tui_big_text::PixelSize::Quadrant)
                    .build();
            let block = Block::new()
                .borders(Borders::ALL)
                .title_top(
                    "Getting Started With Ratatui"
                );
            frame.render_widget(block, frame.area());
            frame.render_widget(title, layout[1]);
            frame.render_widget(subtitle, layout[2]);
        })
        .unwrap();
    event::read().unwrap();
    ratatui::restore();
}
