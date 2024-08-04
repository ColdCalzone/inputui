use inputbot::{KeybdKey, KeybdKey::*, MouseButton::*};
use std::iter;
use std::sync::Arc;
use ratatui::{
    style::Stylize,
    widgets::{Widget, Paragraph, Padding, Block},
    layout::{
        Rect,
        Layout,
        Flex,
        Constraint::{Max, Fill},
    },
    buffer::Buffer,
    style::Style,
};

pub fn render(area: Rect, buf: &mut Buffer) {
    
}
