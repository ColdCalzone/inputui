use std::io::{stdout, Result};
use std::iter;
use std::thread;
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
    layout::{
        Layout,
        Constraint::Percentage
    }
};
use inputbot::{KeybdKey, MouseButton};

pub mod key_display;
pub mod mouse_display;

use key_display::{KeyboardDisplay, KeyObj};

fn main() -> Result<()> {
    let mut keyboard : KeyboardDisplay = KeyboardDisplay::fromKeyObjs(vec![
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

    // let mut keys_pressed = KEYS.iter().map(|x| x.0).zip(iter::repeat(false)).collect::<HashMap<KeybdKey, bool>>();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    for i in 0..keyboard.keys_to_render.len() {
        match &keyboard.keys_to_render[i] {
            KeyObj::RenderedKey { key } => {
                let key_ = key.clone();  
                key_.bind(move || {
                });
            }
            _                           => continue,
        }
    }

    MouseButton::LeftButton.bind(|| {});
    MouseButton::RightButton.bind(|| {});

    let thread = thread::spawn(move || -> Result<()> {
        loop {
            // Render
            terminal.draw(|frame| {
                let area = frame.size();

                let layout = Layout::vertical([
                    Percentage(50),
                    Percentage(50),
                ]).spacing(1).split(area);
                
                frame.render_widget(&keyboard, layout[0]);
                mouse_display::render(layout[1], frame.buffer_mut());
            })?;

            // Input events
            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL) {
                        return Ok(())
                    }
                }
            }
        }
    });

    thread::spawn(|| inputbot::handle_input_events());
    
    thread.join().expect("Error spawning thread");

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    Ok(())
}
