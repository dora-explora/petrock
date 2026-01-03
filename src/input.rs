use std::io::Result;
use ratatui::{
    crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent},
    layout::Position,
};
use crate::App;

impl App  {
    pub fn handle_events(&mut self) -> Result<()> {
        match read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => self.handle_keyevent(event),
            Event::Mouse(event) => self.handle_mousevent(event),
            Event::Resize(width, height) => self.handle_resize(width, height),
            _ => {}
        }
        Ok(())
    }

    fn handle_keyevent(&mut self, event: KeyEvent) {
        match (event.modifiers, event.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('u')) => self.unlock(),
            (_, KeyCode::Up) => self.arrowselection(true),
            (_, KeyCode::Down) => self.arrowselection(false),
            _ => {}
        }
    }

    fn handle_mousevent(&mut self, event: MouseEvent) {
        self.mousepos = Position::new(event.column, event.row);
    }

    fn handle_resize(&mut self, width: u16, height: u16) {
        // if width < 100 || height < 30 {
            // panic!("Please keep the terminal size at or above 100x30"); // this is arbitrary
        // }
    }
}
