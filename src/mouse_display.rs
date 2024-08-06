use inputbot::MouseButton::*;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint::Percentage, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Widget, WidgetRef},
};

#[derive(Default)]
pub struct MouseDisplay {
    // scroll: bool,
}

impl WidgetRef for MouseDisplay {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mouse_layout = Layout::vertical([Percentage(70), Percentage(30)])
            .spacing(1)
            .split(area);

        let buttons_layout = Layout::horizontal([Percentage(50), Percentage(50)])
            .spacing(1)
            .split(mouse_layout[0]);

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
}
