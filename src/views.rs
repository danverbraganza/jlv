use std::io;

use crate::model::Record;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Row, Table, Widget},
};

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

impl Widget for &App {
    // This method renders the specific widgets that we need
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" jlv - {0} ", self.filename).bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::PLAIN);

        let mut row_v: Vec<Row> = vec![];
        for record in &self.records {
            row_v.push(match &record.value {
                Some(value) => record.to_row(RowViewType::ObjSimple),
                None => Row::new(vec![" ", " ", " "]),
            })
        }

        // TODO: Set the table columns from the data
        let table = Table::new(row_v, [20, 20, 20, 20, 20, 20, 20, 20, 20])
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

        match &self.value {
            None => (),
            Some(value) => match value.as_object() {
                None => (),
                Some(object) => {
                    for (_key, value) in object {
                        cells.push(value.to_string())
                    }
                }
            },
        }

        Row::new(cells)
    }
}
