use std::io;

use crate::model::Record;

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
use serde_json::value;

use crate::input::records_from_file;

// Initializes the TUI view to view a given filename
pub fn start_view(filename: &str) -> io::Result<()> {
    println!("Opening filename {filename}");

    // Open the file passed in.
    let records = records_from_file(filename);
    let a = App {
        filename: filename.into(),
        records,
    };

    color_eyre::install().expect("This should install");
    let terminal = ratatui::init();
    let result = a.run(terminal);
    ratatui::restore();
    result
}

struct App {
    filename: Box<str>,
    records: Vec<Record>,
}

impl App {
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

struct TableViewConfig {
    keys: HashMap<String, ColumnConfig>,
}

impl Widget for &App {
    // This method renders the specific widgets that we need
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" jlv - {0} ", self.filename).bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);

        let mut row_v: Vec<Row> = vec![];
        let mut header: Vec<String> = vec![];

        for record in &self.records {
            row_v.push(record.to_row(RowViewType::ObjSimple))
        }

        for record in &self.records {
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
        let table = Table::new(row_v, [20, 20, 20, 20, 20, 20, 20, 20, 20])
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
