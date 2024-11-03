#[allow(unused_mut, unused_variables)]
fn main() {
    // This is a helper function that is part of Ratatui.
    // It will:
    // - Create a new handle to the terminal
    // - Enter 'raw' mode: allows us to read/write
    //   directly to the terminal without interference
    //   from the OS.
    // - Enter an alternate screen: Render to a new page
    //   of the terminal - saving the existing.
    // - Create a 'panic hook' that will ensure
    //   ratatui::restore() is still called if your
    //   application panics.
    let mut terminal = ratatui::init();
}
