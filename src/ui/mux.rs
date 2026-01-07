// Mux is the multiplexer that allows us to add windows, and draws the current window.
use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::model::Record;

use super::{detail::DetailView, table::TableView};

pub struct Mux {
    table_view: TableView,
    possible_tabs: Vec<DetailView>,
    current_index: i16,
}

pub enum TabDirection {
    Forward,
    Backward,
}

impl Mux {
    pub fn new(table_view: TableView) -> Self {
        Self {
            table_view,
            possible_tabs: Vec::new(),
            current_index: -1,
        }
    }

    pub fn handle_keypress(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: crossterm::event::KeyCode::Enter,
                ..
            } => {
                if self.on_table() {
                    let selected_record = self.table_view.selected_record();
                    self.add_detail_view(selected_record);
                } else {
                    self.close_current_tab()
                }
            }

            _ => self.table_view.handle_keypress(key),
        }
    }

    pub fn add_detail_view(&mut self, record: Record) {
        self.possible_tabs.push(DetailView::new(record));
        self.current_index = self.possible_tabs.len() as i16 - 1
    }

    pub fn close_current_tab(&mut self) {
        match self.current_index.into() {
            Some(delindex) if delindex > -1 => {
                self.possible_tabs.remove(delindex as usize);
                self.current_index -= 1
            }
            _ => (),
        }
    }

    pub fn num_tabs(self) -> usize {
        1 + self.possible_tabs.len()
    }

    pub fn on_table(&self) -> bool {
        self.current_index == -1 || self.current_index - 1 > self.possible_tabs.len() as i16
    }

    pub fn switch_tab(&mut self, direction: TabDirection) {
        match direction {
            TabDirection::Forward => {
                if self.current_index < self.possible_tabs.len() as i16 - 1 {
                    self.current_index += 1
                }
            }
            TabDirection::Backward => {
                if self.current_index > -1 {
                    self.current_index -= 1
                }
            }
        }
    }
}

impl Widget for &mut Mux {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.on_table() {
            self.table_view.render(area, buf);
        } else {
            self.possible_tabs.last().unwrap().render(area, buf)
        }
    }
}
