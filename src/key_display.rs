use inputbot::{KeybdKey, KeybdKey::*};
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{Fill, Max},
        Flex, Layout, Rect,
    },
    style::Style,
    style::Stylize,
    widgets::{Block, BorderType, Paragraph, Widget, WidgetRef},
};
use std::{
    iter,
    sync::{Arc, Mutex},
};

pub enum KeyObj {
    Blank,
    RenderedKey {
        key: KeybdKey,
        presses: Arc<Mutex<u64>>,
    },
    Break,
    Fill,
}

impl KeyObj {
    pub fn from_char(letter: char) -> Self {
        KeyObj::RenderedKey {
            key: inputbot::get_keybd_key(letter).expect("Invalid key"),
            presses: Arc::new(Mutex::new(0)),
        }
    }

    pub fn from_key(key: KeybdKey) -> Self {
        KeyObj::RenderedKey {
            key,
            presses: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_paragraph(&self) -> Option<Paragraph> {
        match self {
            KeyObj::RenderedKey { key, .. } => Some(
                Paragraph::new(String::from(match key {
                    SpaceKey => ' ',
                    _ => inputbot::from_keybd_key(*key)
                        .expect("Invalid key")
                        .to_ascii_uppercase(),
                }))
                .centered()
                .block(Block::bordered().border_type(BorderType::Rounded)),
            ),

            _ => None,
        }
    }
}

pub struct KeyboardDisplay {
    pub keys_to_render: Box<[KeyObj]>,
}

impl KeyboardDisplay {
    pub fn from_key_objs<I>(key_objs: I) -> Self
    where
        I: Into<Box<[KeyObj]>>,
    {
        KeyboardDisplay {
            keys_to_render: key_objs.into(),
        }
    }

    pub fn bind_keys(&self) {
        // This is going to be relevant later, and I'd like to be able to READ, thank you very much.
        #[allow(clippy::single_match)]
        self.keys_to_render.iter().for_each(|k| match k {
            KeyObj::RenderedKey { key, presses } => {
                let presses_ = presses.clone();
                key.bind(move || {
                    let mut p = presses_.lock().unwrap();
                    *p += 1;
                });
            }
            _ => (),
        });
    }
}

impl WidgetRef for KeyboardDisplay {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let keys_chunks: Vec<&[KeyObj]> = self
            .keys_to_render
            .split(|k_obj| {
                if let KeyObj::Break = *k_obj {
                    return true;
                }
                false
            })
            .flat_map(|keys| keys.chunks((area.width / 9) as usize))
            .collect();
        let verticals = Layout::vertical(iter::repeat(Max(3)).take(keys_chunks.len()))
            .flex(Flex::SpaceBetween)
            .split(area);
        let verticals_keys = verticals.iter().zip(&keys_chunks);

        for (v_area, keys) in verticals_keys {
            let horizontals = Layout::horizontal(
                (keys.iter().map(|x| match *x {
                    KeyObj::RenderedKey { key, .. } => match key {
                        SpaceKey => Max(48),
                        _ => Max(9),
                    },

                    KeyObj::Blank => Max(9),

                    KeyObj::Fill | KeyObj::Break => Fill(1),
                }))
                .take(keys.len()),
            )
            .flex(Flex::SpaceAround)
            .split(*v_area);
            keys.iter()
                .zip(horizontals.iter())
                .for_each(|(r_key, area)| {
                    if let Some(key_p) = r_key.get_paragraph() {
                        let mut style = Style::new().white();
                        if let KeyObj::RenderedKey { key, .. } = *r_key {
                            style = if key.is_pressed() {
                                style.on_red()
                            } else {
                                style.on_blue()
                            };
                        }

                        key_p.style(style).render(*area, buf);
                    }
                });
        }
    }
}
