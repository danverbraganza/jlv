// Overall interface of the View for this App.

use std::collections::HashMap;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Margin, Offset, Rect},
    style::{Style, Styled, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Row, Table, Widget},
};

use crate::{
    model::{FileRecordSource, RecordSource},
    ui::table::TableView,
};

// Initializes the TUI view to view a given filename
pub fn start_view(filename: &str) -> Result<(), io::Error> {
    let file_record_source = FileRecordSource::open(filename)?;

    // Open the file passed in.
    let a = App::new(Box::new(file_record_source));

    color_eyre::install().expect("This should install");
    let terminal = ratatui::init();
    let result = a.run(terminal);
    ratatui::restore();
    result
}

struct App<'app> {
    record_source: Box<dyn RecordSource + 'app>,
    // This is used to cache/store the calculated table view configuration.
}

impl App<'_> {
    // Creates a new instance of the app. An App will be created using a RecordSource and a Configuration, but for now
    // we use a RecordSource and a standard configuration.
    fn new(record_source: Box<dyn RecordSource>) -> Self {
        Self { record_source }
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
}

impl Widget for &App<'_> {
    // This method renders the specific widgets that we need
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::bordered()
            .title(Line::from(format!(" jlv - {0} ", self.record_source.title()).bold()).centered())
            .border_set(border::PLAIN);
        main_block.render(area, buf);

        // TODO: Put the view Muxer here.
        let mut table_view = TableView::new(self.record_source.as_ref());
        table_view.update_config();
        table_view.render(
            area.inner(Margin::new(1, 3)).offset(Offset { x: 0, y: -2 }),
            buf,
        );
    }
}
