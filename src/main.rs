use std::io::{stdout, Result};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

pub mod key_display;

use key_display::RenderedKey;

fn main() -> Result<()> {
    let KEYS : [RenderedKey; 6] = [
        RenderedKey::new('a'),
        RenderedKey::new('b'),
        RenderedKey::new('c'),
        RenderedKey::new('d'),
        RenderedKey::new('e'),
        RenderedKey::new('f'),
    ];

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        // Input events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('c') || key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL) {
                    break;
                }
            }
        }

        // Render
        terminal.draw(|frame| {
            let area = frame.size();
            key_display::render(area, &KEYS, frame.buffer_mut());
        })?;
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
