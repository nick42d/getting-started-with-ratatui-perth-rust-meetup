use ratatui::{
    crossterm::event,
    widgets::{Block, Borders},
};

fn main() {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
    terminal
        .draw(|frame| {
            let block = Block::new()
                .borders(Borders::ALL)
                .title_top("Getting Started With Ratatui");
            frame.render_widget(block, frame.area());
        })
        .unwrap();
    event::read().unwrap();
    ratatui::restore();
}
