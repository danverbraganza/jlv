use std::io;

use color_eyre::Result as cResult;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::input::records_from_file;

// Initializes the TUI view to view a given filename
pub fn start_view(filename: &str) -> io::Result<()> {
    println!("Opening filename {filename}");

    // Open the file passed in.
    let records = records_from_file(filename);

    for (i, record) in records.iter().enumerate() {
        println!("{}: {:#?}", i, record)
    }

    color_eyre::install().expect("This should install");

    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}
