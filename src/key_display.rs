use inputbot::{KeybdKey, KeybdKey::*, MouseButton::*};
use std::io::{stdout, Result};
use std::iter;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::{Widget, Paragraph, Padding, Block},
    Terminal,
    layout::{
        Rect,
        Layout,
        Flex,
        Constraint::{self, Fill, Max},
        Alignment,
    },
    buffer::Buffer,
    style::Style,
};

pub struct RenderedKey(KeybdKey);

impl RenderedKey {
    pub fn new(letter: char) -> Self {
        RenderedKey(inputbot::get_keybd_key(letter).expect("Invalid key"))
    }

    pub fn get_paragraph(&self) -> Paragraph {
        Paragraph::new(String::from(inputbot::from_keybd_key(self.0).expect("Invalid key").to_ascii_uppercase()))
            .centered()
            .block(Block::new().padding(Padding::vertical(1)))
    }
}


pub fn render(area: Rect, keys_to_render : &[RenderedKey], buf: &mut Buffer) {
    let keys_chunks = keys_to_render.chunks((area.width / 10) as usize);
    let verticals = Layout::vertical(iter::repeat(Max(3)).take(keys_chunks.len())).flex(Flex::SpaceAround).split(area);
    let verticals_keys = verticals.into_iter().zip(keys_chunks);

    for (v_area, keys) in verticals_keys {
        let horizontals = Layout::horizontal(iter::repeat(Max(9)).take(keys.len())).flex(Flex::SpaceAround).split(*v_area);

        keys.iter().zip(horizontals.iter()).for_each(|(r_key, area)| {
            let key = r_key.get_paragraph();

            let mut style = Style::new().white();
            style = if false {
                style.on_red()
            } else {
                style.on_blue()
            };


            key.style(style).render(*area, buf);
        });
    }
}
