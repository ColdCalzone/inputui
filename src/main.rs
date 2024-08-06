use inputbot::{KeybdKey, MouseButton};
use mouse_display::MouseDisplay;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint::Percentage, Layout},
    Terminal,
};
use std::io::{stdout, Result};
use std::thread;

mod key_display;
mod mouse_display;

use key_display::{KeyObj, KeyboardDisplay};

fn main() -> Result<()> {
    let keyboard: KeyboardDisplay = KeyboardDisplay::from_key_objs(vec![
        KeyObj::Blank,
        KeyObj::from_char('W'),
        KeyObj::from_char('E'),
        KeyObj::from_char('R'),
        KeyObj::Break,
        KeyObj::from_char('A'),
        KeyObj::from_char('S'),
        KeyObj::from_char('D'),
        KeyObj::Blank,
        KeyObj::Break,
        KeyObj::from_key(KeybdKey::SpaceKey),
    ]);

    keyboard.bind_keys();

    let mouse: MouseDisplay = MouseDisplay::default();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    MouseButton::LeftButton.bind(|| {});
    MouseButton::RightButton.bind(|| {});

    let thread = thread::spawn(move || -> Result<()> {
        loop {
            // Render
            terminal.draw(|frame| {
                let area = frame.size();

                let layout = Layout::vertical([Percentage(50), Percentage(50)])
                    .spacing(1)
                    .split(area);

                frame.render_widget(&keyboard, layout[0]);
                frame.render_widget(&mouse, layout[1]);
            })?;

            // Input events
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && (key.code == KeyCode::Char('c')
                            && key.modifiers == KeyModifiers::CONTROL)
                    {
                        return Ok(());
                    }
                }
            }
        }
    });

    thread::spawn(inputbot::handle_input_events);

    thread.join().expect("Error spawning thread")?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
