use std::io::{Result, stdout};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        ExecutableCommand,
        event::{DisableMouseCapture, EnableMouseCapture},
    },
    layout::Position,
};
mod upgrades;
use crate::upgrades::*;
mod render;
mod input;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    return result;
}

struct App {
    mousepos: Position, // current position of the mouse
    infotext: String, // text at the bottom of the screen
    upgrades: [Upgrade; NUMUPGRADES], // every upgrade in the game
    purchases: [usize; NUMUPGRADES], // count of purchases for each upgrade
    unlocked: usize, // number of currently unlocked upgrades
    selected: usize, // currently selected upgrade
    running: bool, // whether or not the app is running
}

impl App {
    fn new() -> App {
        return App {
            mousepos: Position { x: 0, y: 0 },
            infotext: String::from("Test!"),
            upgrades: upgrades(),
            purchases: [0; NUMUPGRADES],
            unlocked: 0,
            selected: 0,
            running: true
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        terminal.clear()?;
        let size = terminal.size()?;
        // if size.width < 100 || size.height < 30 {
            // panic!("Please keep the terminal size at or above 100x30"); // this is arbitrary
        // }
        stdout().execute(EnableMouseCapture)?;
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        stdout().execute(DisableMouseCapture)?;
        return Ok(())
    }

    fn unlock(&mut self) {
        let mut unlocked = self.unlocked;
        unlocked += 1;
        if unlocked > NUMUPGRADES {
            unlocked = NUMUPGRADES;
        } else {
            self.infotext = format!("You've unlocked upgrade number {}: {}!\n{}", unlocked, self.upgrades[unlocked - 1].title, self.upgrades[unlocked - 1].description);
        }
        self.unlocked = unlocked;
    }

    fn select(&mut self, selection: usize) {
        self.selected = selection;
        self.infotext = self.upgrades[selection].render().to_string();
    }

    fn arrowselection(&mut self, direction: bool) {
        let mut selection = self.selected;
        if self.unlocked == 0 { return; }
        if direction {
            if self.selected == 0 { selection = self.unlocked - 1; }
            else { selection -= 1; }
        } else {
            selection += 1;
            if selection == self.unlocked { selection = 0; }
        }
        self.select(selection);
    }

    fn mouseselect(&mut self, mousepos: Position) {

    }
}
