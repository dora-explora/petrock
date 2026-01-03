use std::io::{Result, stdout};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        ExecutableCommand,
        event::{DisableMouseCapture, EnableMouseCapture},
    },
    layout::{Position, Size},
};
mod upgrades;
use crate::upgrades::*;
mod render;
mod input;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    let result = App::new(terminal.size()?).run(terminal);
    ratatui::restore();
    return result;
}

struct App {
    pets: usize, // current pets (currency)
    pps: usize, // current total pets per second
    mousepos: Position, // current position of the mouse
    rockpos: Position, // rendered position of the rock (for clicking)
    infotext: String, // text at the bottom of the screen
    upgrades: [Upgrade; NUMUPGRADES], // every upgrade in the game
    purchases: [usize; NUMUPGRADES], // count of purchases for each upgrade
    unlocked: usize, // number of currently unlocked upgrades
    selection: usize, // currently selected upgrade
    listoffset: usize, // upgrade list item offset (as reported by ListState)
    size: Size, // current resolution
    running: bool, // whether or not the app is running
}

impl App {
    fn new(size: Size) -> App {
        return App {
            pets: 0,
            pps: 0,
            mousepos: Position::new(0, 0),
            rockpos: Position::new(0, 0),
            infotext: String::from("Test!"),
            upgrades: upgrades(),
            purchases: [0; NUMUPGRADES],
            unlocked: 0,
            selection: 0,
            listoffset: 0,
            size,
            running: true
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        terminal.clear()?;
        // if self.size.width < 100 || self.size.height < 30 {
            // panic!("Please keep the terminal size at or above 100x30"); // this is arbitrary
        // }
        stdout().execute(EnableMouseCapture)?;
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
            self.tick();
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
        self.selection = selection;
        self.infotext = self.upgrades[selection].render(self.purchases[selection]).to_string();
    }

    fn arrowselection(&mut self, direction: bool) {
        let mut selection = self.selection;
        if self.unlocked == 0 { return; }
        if direction {
            if self.selection == 0 { selection = self.unlocked - 1; }
            else { selection -= 1; }
        } else {
            selection += 1;
            if selection == self.unlocked { selection = 0; }
        }
        self.select(selection);
    }

    fn mouseselect(&mut self) {
        let selection: usize = ((self.mousepos.y - 2) / 4) as usize;
        if selection + self.listoffset < self.unlocked { self.select(selection + self.listoffset); }
    }

    fn buy(&mut self) {
        let upgrade = self.upgrades[self.selection].clone();
        let purchases = self.purchases[self.selection];
        let cost = upgrade.cost + upgrade.factor * (purchases * (purchases + 1)) / 2; // thank you math class
        if self.pets >= cost {
            self.pets -= cost;
            self.pps += upgrade.pps;
            self.purchases[self.selection] += 1;
            self.infotext = format!("{} purchased!", upgrade.title)
        } else {
            self.infotext = format!("You can't afford a {}!", upgrade.title);
        }
    }

    fn tick(&mut self) {
        self.pets += self.pps;
        if self.unlocked < NUMUPGRADES {
            if self.upgrades[self.unlocked].cost <= self.pets {
                self.unlocked += 1;
            }
        }
    }

    fn pet(&mut self) {
        self.pets += 1;
    }
}
