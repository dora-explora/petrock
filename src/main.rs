use std::io::{Result, stdout};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        ExecutableCommand,
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent}
    },
    layout::{Position, Rect},
    text::{Text},
    widgets::{Block, BorderType, Widget}
};

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    return result;
}

struct App {
    mousepos: Position,
    running: bool,
}

impl App {
    fn new() -> App {
        return App {
            mousepos: Position { x: 0, y: 0 },
            running: true
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        terminal.clear()?;
        let size = terminal.size()?;
        if size.width < 100 || size.height < 30 {
            panic!("Please keep the terminal size at or above 100x30"); // this is currently arbitrary
        }
        stdout().execute(EnableMouseCapture)?;
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        stdout().execute(DisableMouseCapture)?;
        return Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(Block::bordered(), frame.area());
        frame.render_widget(Text::from("x"), Rect::new(self.mousepos.x, self.mousepos.y, 1, 1));
    }

    fn render_sidebar(&self) -> impl Widget {
        return Block::bordered().border_type(BorderType::Rounded);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
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
            _ => {}
        }
    }

    fn handle_mousevent(&mut self, event: MouseEvent) {
        self.mousepos = Position::new(event.column, event.row);
    }

    fn handle_resize(&mut self, width: u16, height: u16) {
        if width < 100 || height < 30 {
            panic!("Please keep the terminal size at or above 100x30");
        }
    }
}
