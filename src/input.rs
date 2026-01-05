use std::{io::Result, time::Duration};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind, poll, read},
    layout::{Position, Size},
};
use crate::App;

impl App  {
    pub fn handle_events(&mut self) -> Result<()> {
        if poll(Duration::from_millis(32))? {
            match read()? {
                Event::Key(event) if event.kind == KeyEventKind::Press => self.handle_keyevent(event),
                Event::Mouse(event) => self.handle_mousevent(event),
                Event::Resize(width, height) => self.handle_resize(width, height),
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_keyevent(&mut self, event: KeyEvent) {
        match (event.modifiers, event.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Enter) => self.buy(),
            (_, KeyCode::Up) => self.arrowselection(true),
            (_, KeyCode::Down) => self.arrowselection(false),
            (_, KeyCode::Char(' ')) => self.pet(), // this can be abused unfortunately
            _ => {}
        }
    }

    fn handle_mousevent(&mut self, event: MouseEvent) {
        self.mousepos = Position::new(event.column, event.row);
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                if self.size.width - self.mousepos.x <= 60 && self.mousepos.y < self.size.height * 3 / 4 {
                    self.mouseselect();
                    return;
                }
                let deltax: isize = self.mousepos.x as isize - self.rockpos.x as isize;
                let deltay: isize = self.mousepos.y as isize - self.rockpos.y as isize;
                if deltax*deltax/8 + deltay*deltay < 40 { self.pet(); }
            },
            MouseEventKind::Down(MouseButton::Right) => {
                if self.size.width - self.mousepos.x <= 60 && self.mousepos.y < self.size.height * 3 / 4 {
                    self.mouseselect();
                    self.buy();
                }
            },
            MouseEventKind::ScrollUp => { self.arrowselection(true); },
            MouseEventKind::ScrollDown => { self.arrowselection(false); },
            _ => {}
        }
    }

    fn handle_resize(&mut self, width: u16, height: u16) {
        self.size = Size::new(width, height);
        // if width < 100 || height < 30 {
            // panic!("Please keep the terminal size at or above 100x30"); // this is arbitrary
        // }
    }
}
