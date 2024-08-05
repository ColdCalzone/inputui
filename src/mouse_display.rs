use inputbot::{MouseButton, MouseButton::*};
use std::iter;
use std::sync::Arc;
use ratatui::{
    style::Stylize,
    widgets::{Widget, Paragraph, Padding, Block},
    layout::{
        Rect,
        Layout,
        Flex,
        Constraint::{Max, Fill, Percentage},
    },
    buffer::Buffer,
    style::Style,
};

pub fn render(area: Rect, buf: &mut Buffer) {
    let mouse_layout = Layout::vertical([Percentage(70), Percentage(30)]).spacing(1).split(area);

    let buttons_layout = Layout::horizontal([Percentage(50), Percentage(50)]).spacing(1).split(mouse_layout[0]);

    for (area, mb) in buttons_layout.iter().zip([LeftButton, RightButton]) {
        Block::new()
            .style(if mb.is_pressed() {
                    Style::new().on_red()
                } else {
                    Style::new().on_blue()
                })
            .render(*area, buf);
    }

    Block::new()
        .style(Style::new().on_blue())
        .render(mouse_layout[1], buf);
}
