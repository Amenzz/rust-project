use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();

        // Create a centered block with a greeting message
        let block = Paragraph::new("Hello, Ratatui!")
            .block(Block::default().borders(Borders::ALL).title("Greeting"))
            .alignment(Alignment::Center);

        f.render_widget(block, size);
    })?;

    Ok(())
}
