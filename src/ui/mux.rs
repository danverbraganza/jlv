// Mux is the multiplexer that allows us to add windows, and draws the current window.

use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use super::table::TableView;

pub struct Mux {
    table_view: TableView,
    //possible_tabs: Vec<>,
}

impl Mux {
    pub fn new(table_view: TableView) -> Mux {
        Self { table_view }
    }

    pub fn handle_keypress(&mut self, key: KeyEvent) {
        self.table_view.handle_keypress(key);
    }
}

impl Widget for &mut Mux {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.table_view.render(area, buf);
    }
}
