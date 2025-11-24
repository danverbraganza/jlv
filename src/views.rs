use std::io;

use color_eyre::Result as cResult;

use crate::model::Record;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListDirection, Paragraph, Table, Widget},
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

        let mut str_v: Vec<String> = vec![];
        for record in &self.records {
            str_v.push(match &record.value {
                Some(value) => format!("{:#?}", value),
                None => "".to_string(),
            })
        }

        let list = List::new(str_v)
            .block(block)
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        list.render(area, buf);
    }
}
