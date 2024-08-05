use inputbot::{KeybdKey, KeybdKey::*, MouseButton::*};
use std::iter;
use std::sync::Arc;
use ratatui::{
    style::Stylize,
    widgets::{Widget, WidgetRef, Paragraph, Padding, Block, BorderType},
    layout::{
        Rect,
        Layout,
        Flex,
        Constraint::{Max, Fill},
    },
    buffer::Buffer,
    style::Style,
};

#[derive(PartialEq)]
pub enum KeyObj {
    Blank,
    RenderedKey {
      key: Arc<KeybdKey>,
    },
    Break,
    Fill,
}

impl KeyObj {
    pub fn from_char(letter: char) -> Self {
        KeyObj::RenderedKey {
            key: inputbot::get_keybd_key(letter).expect("Invalid key").into(),
        }
    }

    pub fn from_key(key: KeybdKey) -> Self {
        KeyObj::RenderedKey {
            key: key.into(),
        }
    }

    pub fn get_paragraph(&self) -> Option<Paragraph> {
        match self {
            KeyObj::RenderedKey { key } => Some(Paragraph::new(String::from(match **key {
                SpaceKey => ' ',
                _        => inputbot::from_keybd_key(**key).expect("Invalid key").to_ascii_uppercase(),
            }))
            .centered()
            .block(Block::bordered().border_type(BorderType::Rounded))),

            _                           => None,
        }
    }
}

pub struct KeyboardDisplay {
    pub keys_to_render : Vec<KeyObj>
}

impl KeyboardDisplay {
    pub fn fromKeyObjs(vec : Vec<KeyObj>) -> Self {
        KeyboardDisplay {
            keys_to_render: vec
        }
    }
}

impl WidgetRef for KeyboardDisplay {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let keys_chunks : Vec<&[KeyObj]> = self.keys_to_render.split(|k_obj| k_obj == &KeyObj::Break).flat_map(|keys| keys.chunks((area.width / 9) as usize)).collect();
        let verticals = Layout::vertical(iter::repeat(Max(3)).take(keys_chunks.len())).flex(Flex::SpaceBetween).split(area);
        let verticals_keys = verticals.into_iter().zip(&keys_chunks);

        for (v_area, keys) in verticals_keys {
            let horizontals = Layout::horizontal((keys.iter().map(|x| 
                match x {
                    KeyObj::RenderedKey { key } => {
                        match *(key.clone()) {
                            SpaceKey => Max(48),
                            _        => Max(9),
                        }
                    },

                    KeyObj::Blank => {
                        Max(9)
                    },

                    KeyObj::Fill | KeyObj::Break => {
                        Fill(1)
                    }
                })).take(keys.len())).flex(Flex::SpaceAround).split(*v_area);
            keys.iter().zip(horizontals.iter()).for_each(|(r_key, area)| {
                if let KeyObj::Blank = r_key {
                    return;
                }
                else if let Some(key_p) = r_key.get_paragraph() {
        
                    let mut style = Style::new().white();
                    if let KeyObj::RenderedKey { key } = r_key {
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
