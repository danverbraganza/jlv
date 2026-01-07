// This module allows you to visualize a single record as a JSON string.

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Paragraph, Widget},
};
use serde_json::to_string_pretty;

use crate::model::Record;

// DetailView is a Widget that renders a single Records.
pub struct DetailView {
    // TODO: This should be possible to not copy!
    record: Record,
}

impl DetailView {
    pub fn new(record: Record) -> Self {
        Self { record }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Text::raw(
            to_string_pretty(
                self.record
                    .value
                    .as_ref()
                    .unwrap_or(&serde_json::Value::Null),
            )
            .unwrap_or("".to_string()),
        ))
        .left_aligned()
        .render(area, buf);
    }

    pub fn handle_keypress(&mut self, _key: KeyEvent) {}
}
