// Mux is the multiplexer that allows us to add windows, and draws the current window.
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Offset, Rect},
    symbols::{self, border},
    widgets::{Block, Tabs, Widget},
};

use crate::model::Record;

use super::{detail::DetailView, table::TableView};

pub struct Mux {
    table_view: TableView,
    possible_tabs: Vec<DetailView>,
    current_index: i16,
}

pub enum TabDestination {
    Forward,
    Backward,
    Home,
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
            KeyEvent {
                code: KeyCode::F(8),
                ..
            } => self.switch_tab(TabDestination::Forward),
            KeyEvent {
                code: KeyCode::F(6),
                ..
            } => self.switch_tab(TabDestination::Backward),

            KeyEvent {
                code: KeyCode::F(5),
                ..
            } => self.switch_tab(TabDestination::Home),

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

    pub fn num_tabs(&self) -> usize {
        1 + self.possible_tabs.len()
    }

    pub fn on_table(&self) -> bool {
        self.current_index == -1 || self.current_index - 1 > self.possible_tabs.len() as i16
    }

    pub fn switch_tab(&mut self, direction: TabDestination) {
        match direction {
            TabDestination::Forward => {
                if self.current_index < self.possible_tabs.len() as i16 - 1 {
                    self.current_index += 1
                }
            }
            TabDestination::Backward => {
                if self.current_index > -1 {
                    self.current_index -= 1
                }
            }
            TabDestination::Home => self.current_index = -1,
        }
    }
}

impl Widget for &mut Mux {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_area = area.inner(Margin::new(1, 3)).offset(Offset { x: 0, y: -2 });

        if self.on_table() {
            self.table_view.render(inner_area, buf);
        } else {
            self.possible_tabs.last().unwrap().render(inner_area, buf)
        }

        let mut v: Vec<String> = vec![
            "F1".to_string(),
            "Prev (F6)".to_string(),
            "Next (F8)".to_string(),
            "Table (F5)".to_string(),
        ];

        let x = self.num_tabs() - 1;
        for i in 0..x {
            v.push(format!("Detail {}", i + 1));
        }

        v.push("Exit (q)".to_string());

        Tabs::new(v)
            .block(Block::bordered().border_set(border::PLAIN))
            .divider(symbols::DOT)
            .select((self.current_index + 4).try_into().unwrap_or(4))
            .render(
                Rect {
                    x: area.x,
                    y: area.bottom() - 3,
                    width: area.width,
                    height: 3,
                },
                buf,
            );
    }
}
