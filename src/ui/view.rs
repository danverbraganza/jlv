// Overall interface of the View for this App.

use std::clone::Clone;
use std::io;
use std::rc::Rc;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Margin, Offset, Rect},
    style::Stylize,
    symbols::{self, border},
    text::Line,
    widgets::{Block, Tabs, Widget},
};

use crate::{
    model::{FileRecordSource, RecordSource},
    ui::table::TableView,
};

use super::mux::Mux;

// Initializes the TUI view to view a given filename
pub fn start_view(filename: &str) -> Result<(), io::Error> {
    let file_record_source = FileRecordSource::open(filename)?;

    // Open the file passed in.
    let mut a = App::new(Box::new(file_record_source));

    color_eyre::install().expect("This should install");
    let terminal = ratatui::init();
    let result = a.run(terminal);
    ratatui::restore();
    result
}

struct App {
    record_source: Rc<Box<dyn RecordSource>>,
    // This is used to cache/store the calculated table view configuration.
    mux: Mux,
}

impl App {
    // Creates a new instance of the app. An App will be created using a RecordSource and a Configuration, but for now
    // we use a RecordSource and a standard configuration.
    fn new(record_source: Box<dyn RecordSource>) -> Self {
        let r = Rc::new(record_source);
        let s = r.clone();

        let mut table_view = TableView::new(s);

        table_view.update_config();

        let mux = Mux::new(table_view);

        Self {
            record_source: r,
            mux,
        }
    }

    // Starts the view, and runs until keypress.
    fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q') | KeyCode::Char('Q'),
                    ..
                }) => break Ok(()),
                Event::Key(key) => self.handle_keypress(key),
                _ => (),
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_keypress(&mut self, key: KeyEvent) {
        self.mux.handle_keypress(key);
    }
}

impl Widget for &mut App {
    // This method renders the specific widgets that we need
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::bordered()
            .title(Line::from(format!(" jlv - {0} ", self.record_source.title()).bold()).centered())
            .border_set(border::DOUBLE);

        self.mux.render(
            area.inner(Margin::new(1, 3)).offset(Offset { x: 0, y: -2 }),
            buf,
        );

        // TODO: Derive the Tabs
        Tabs::new(vec!["F1", "F2", "F10", "Exit (q)"])
            .block(Block::bordered().border_set(border::PLAIN))
            .divider(symbols::DOT)
            .render(
                Rect {
                    x: area.x,
                    y: area.bottom() - 3,
                    width: area.width,
                    height: 3,
                },
                buf,
            );

        main_block.render(area, buf);
    }
}
