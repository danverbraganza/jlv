use std::io;

use crate::model::{FileRecordSource, Record, RecordSource};

use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Style, Styled, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Row, Table, Widget},
};

// Initializes the TUI view to view a given filename
pub fn start_view(filename: &str) -> Result<(), io::Error> {
    let file_record_source = FileRecordSource::open(filename)?;

    // Open the file passed in.
    let mut a = App::new(Box::new(file_record_source));
    a.calculate_table_view_config();

    color_eyre::install().expect("This should install");
    let terminal = ratatui::init();
    let result = a.run(terminal);
    ratatui::restore();
    result
}

// Lets create a type whose job it is to record: what keys we've seen, how wide they are, and in what order they should
// be generated.
//
// This type will _start_ off being read only (initialized once), but then will quickly grow to have the power to
// read/sample Rows to update itself.

struct ColumnConfig {
    min_width: i32,
    // TODO: We'll add max_width when we're sampling.
    //     max_width: i32,
    index: i32,
}

// TableViewConfig is the calculated configuration for rendering a table view of Records.
struct TableViewConfig {
    keys: HashMap<String, ColumnConfig>,
}

impl TableViewConfig {
    // Returns the columns we have seen so far, in order.
    fn ordered_columns(&self) -> Vec<(&String, &ColumnConfig)> {
        let mut cols: Vec<(&String, &ColumnConfig)> = self.keys.iter().collect();
        cols.sort_by_key(|(_k, v)| v.index);
        cols
    }

    // Returns the widths of each column in column order.
    fn widths(&self) -> Vec<u16> {
        let mut widths: Vec<u16> = vec![];

        for index in self.ordered_columns() {
            widths.push(index.1.min_width as u16);
        }

        widths
    }
}

struct App {
    record_source: Box<dyn RecordSource>,
    // This is used to cache/store the calculated table view configuration.
    table_view_config: Option<TableViewConfig>,
}

impl App {
    // Creates a new instance of the app. An App will be created using a RecordSource and a Configuration, but for now
    // we use a RecordSource and a standard configuration.
    fn new(record_source: Box<dyn RecordSource>) -> Self {
        Self {
            record_source,
            table_view_config: None,
        }
    }

    // Starts the view, and runs until keypress.
    fn run(self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if matches!(event::read()?, Event::Key(_)) {
                break Ok(());
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    // Calculates the table view configuration, if it hasn't already been calculated.
    fn calculate_table_view_config(&mut self) -> &TableViewConfig {
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
                keys: table_view_config,
            }
        })
    }

    fn get_table_view_config(&self) -> &TableViewConfig {
        self.table_view_config
            .as_ref()
            .expect("TableViewConfig should have been calculated")
    }
}

impl Widget for &App {
    // This method renders the specific widgets that we need
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(format!(" jlv - {0} ", self.record_source.title()).bold()).centered())
            .border_set(border::PLAIN);

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

        // TODO: Set the table columns from the data
        let table = Table::new(row_v, self.get_table_view_config().widths())
            .header(Row::new(header).set_style(Style::new().bold().bg(ratatui::style::Color::Blue)))
            .block(block)
            .style(Style::new().white())
            .row_highlight_style(Style::new().italic())
            .highlight_symbol(">>");

        table.render(area, buf);
    }
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
