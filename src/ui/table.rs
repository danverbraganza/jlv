// This module contains the types and code related to visualizing records as a Table.

// Lets create a type whose job it is to record: what keys we've seen, how wide they are, and in what order they should
// be generated.
//
// This type will _start_ off being read only (initialized once), but then will quickly grow to have the power to
// read/sample Rows to update itself.

use std::{collections::HashMap, rc::Rc};

use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled, Stylize},
    widgets::{Row, StatefulWidget, Table, TableState},
};

use crate::model::{Record, RecordSource};

pub struct ColumnConfig {
    pub min_width: i32,
    // TODO: We'll add max_width when we're sampling.
    //     max_width: i32,
    pub index: i32,
}

pub enum RowViewType {
    // ObjSimple just renders each top-level key as a single cell
    ObjSimple,
}

// RowAble is the trait for an object that can be renderded as a Row within a table.
pub trait RowAble {
    fn to_row<'a>(&self, r: RowViewType) -> Row<'a>;
}

impl RowAble for Record {
    fn to_row<'a>(&self, r: RowViewType) -> Row<'a> {
        let RowViewType::ObjSimple = r else {
            panic!("No other RowViewTypes implemented")
        };

        let mut cells = vec![];

        match self.value.as_ref().and_then(|f| f.as_object()) {
            None => (),
            Some(object) => {
                for (_key, value) in object {
                    cells.push(value.to_string())
                }
            }
        }

        Row::new(cells)
    }
}

// TableViewConfig is the calculated configuration for rendering a table view of Records.
pub struct TableViewConfig {
    pub columns: HashMap<String, ColumnConfig>,
}

impl TableViewConfig {
    // Returns the columns we have seen so far, in order.
    pub fn ordered_columns(&self) -> Vec<(&String, &ColumnConfig)> {
        let mut cols: Vec<(&String, &ColumnConfig)> = self.columns.iter().collect();
        cols.sort_by_key(|(_k, v)| v.index);
        cols
    }

    // Returns the widths of each column in column order.
    pub fn widths(&self) -> Vec<u16> {
        let mut widths: Vec<u16> = vec![];

        for index in self.ordered_columns() {
            widths.push(index.1.min_width as u16);
        }

        widths
    }
}

// Table view is a Widget that renders a slice of Records as Rows, according to some configuration.
pub struct TableView {
    record_source: Rc<Box<dyn RecordSource>>,
    table_view_config: Option<TableViewConfig>,
    pub table_state: TableState,
}

impl TableView {
    pub fn new(record_source: Rc<Box<dyn RecordSource>>) -> Self {
        TableView {
            table_view_config: None,
            record_source,
            table_state: TableState::default().with_selected(3),
        }
    }

    // Calculates the column configuration of this view, if it has not been already.
    pub fn update_config(&mut self) -> &TableViewConfig {
        self.table_view_config.get_or_insert_with(|| {
            let mut table_view_config: HashMap<String, ColumnConfig> = HashMap::new();
            for record in self.record_source.iter() {
                match record.value.as_ref().and_then(|f| f.as_object()) {
                    None => (),
                    Some(value) => {
                        for (key, value) in value {
                            let next_index = table_view_config.len() as i32;
                            let entry =
                                table_view_config
                                    .entry(key.to_string())
                                    .or_insert(ColumnConfig {
                                        min_width: 0,
                                        index: next_index,
                                    });

                            let value_len = value.to_string().len() as i32;
                            if value_len > entry.min_width {
                                entry.min_width = value_len;
                            }

                            // TODO: Optimize this, we should be able to only calculate this once.
                            let key_len = key.to_string().len() as i32;
                            if key_len > entry.min_width {
                                entry.min_width = key_len;
                            }
                        }
                    }
                }
            }

            TableViewConfig {
                columns: table_view_config,
            }
        })
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let mut row_v: Vec<Row> = vec![];
        let mut header: Vec<String> = vec![];

        for record in self.record_source.iter() {
            row_v.push(record.to_row(RowViewType::ObjSimple))
        }

        for record in self.record_source.iter() {
            match record.value.as_ref().and_then(|f| f.as_object()) {
                None => (),
                Some(value) => {
                    for key in value.keys() {
                        header.push(key.to_string())
                    }
                }
            }
        }

        StatefulWidget::render(
            Table::new(
                row_v,
                self.table_view_config
                    .as_ref()
                    .expect("Widths should have been calculated")
                    .widths(),
            )
            .header(Row::new(header).set_style(Style::new().bold().bg(ratatui::style::Color::Blue)))
            .style(Style::new().white())
            .row_highlight_style(Style::new().bg(ratatui::style::Color::Magenta)),
            area,
            buf,
            &mut self.table_state,
        );
    }

    pub fn handle_keypress(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: crossterm::event::KeyCode::Down,
                ..
            } => {
                self.table_state = self
                    .table_state
                    .clone()
                    .with_selected(self.table_state.selected().unwrap_or(0) + 1)
            }
            KeyEvent {
                code: crossterm::event::KeyCode::Up,
                ..
            } => {
                let old_idx = self.table_state.selected().unwrap_or(10);
                let new_idx = if old_idx == 0 { 0 } else { old_idx - 1 };

                self.table_state = self.table_state.clone().with_selected(new_idx);

                // print!("{0:#?}", self.table_state);
            }
            _ => (),
        }
    }
}

impl TableView {
    // TODO: Make this properly non-copy because Record is immutable
    pub fn selected_record(&self) -> Record {
        self.record_source.records()[self.table_state.selected().unwrap_or(0)].clone()
    }
}
