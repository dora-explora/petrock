use std::{
    fs::File,
    io::{Result, stdout},
    time::Instant
};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        ExecutableCommand,
        event::{DisableMouseCapture, EnableMouseCapture},
    },
    layout::{Position, Size},
};
use rodio::{Decoder, Sink, OutputStreamBuilder, Source};

mod upgrades;
use crate::upgrades::*;
mod render;
mod input;

fn main() -> Result<()> {
    let mut output_stream = OutputStreamBuilder::open_default_stream().expect("Could not open default audio stream");
    output_stream.log_on_drop(false);
    let music_source = Decoder::try_from(File::open("./sounds/music.mp3").unwrap()).unwrap().repeat_infinite();
    let sink = Sink::connect_new(output_stream.mixer());
    sink.append(music_source);
    let terminal = ratatui::init();
    let result = App::new(terminal.size()?, sink).run(terminal);
    ratatui::restore();
    return result;
}

struct App {
    pets: usize, // current pets (currency)
    pps: usize, // current total pets per second
    ppc: usize, // current pets per click
    mousepos: Position, // current position of the mouse
    rockpos: Position, // rendered position of the rock (for clicking)
    blushing: usize, // frame timer of how long rock is blushing (0 is not)
    infotext: String, // text at the bottom of the screen
    upgrades: [Upgrade; NUMUPGRADES], // every upgrade in the game
    purchases: [usize; NUMUPGRADES], // count of purchases for each upgrade
    unlocked: usize, // number of currently unlocked upgrades
    selection: usize, // currently selected upgrade
    listoffset: usize, // upgrade list item offset (as reported by ListState)
    size: Size, // current resolution
    lasttick: Instant, // timestamp of last tick
    manualpets: usize, // total number of manual pets this second (for tpps calculation)
    tpps: usize, // total pets per second, calculated every second
    sink: Sink, // music sink thingy
    volume: f32, // volume of the music
    running: bool, // whether or not the app is running
}

impl App {
    fn new(size: Size, sink: Sink) -> App {
        return App {
            pets: 0,
            pps: 0,
            ppc: 1,
            mousepos: Position::new(0, 0),
            rockpos: Position::new(0, 0),
            blushing: 0,
            infotext: String::from("Hello! This is your Pet Rock, Rock! Try petting Rock!\n(Click on Rock [or press Space] to pet them)"),
            upgrades: upgrades(),
            purchases: [0; NUMUPGRADES],
            unlocked: 0,
            selection: 0,
            listoffset: 0,
            size,
            lasttick: Instant::now(),
            manualpets: 0,
            tpps: 0,
            sink,
            volume: 1.,
            running: true
        };
    }

    fn quit(&mut self) {
        self.running = false;
        self.sink.stop();
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
            if self.lasttick.elapsed().as_millis() > 1000 {
                self.lasttick = Instant::now();
                self.tick();
            }
            if self.blushing > 0 { self.blushing -= 1; }
        }
        stdout().execute(DisableMouseCapture)?;
        return Ok(())
    }

    fn unlock(&mut self) {
        let mut unlocked = self.unlocked;
        unlocked += 1;
        if unlocked > NUMUPGRADES {
            unlocked = NUMUPGRADES;
        } else if self.pets >= 25 {
            self.infotext = format!("You've unlocked upgrade number {}: {}!\n{}", unlocked, self.upgrades[unlocked - 1].title, self.upgrades[unlocked - 1].description);
        }
        self.unlocked = unlocked;
    }

    fn select(&mut self, selection: usize) {
        self.selection = selection;
        self.infotext = self.upgrades[selection].render(self.purchases[selection]).to_string();
    }

    fn arrowselect(&mut self, direction: bool) {
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
        let cost = upgrade.cost(purchases);
        if self.pets >= cost {
            self.pets -= cost;
            if upgrade.hand { self.ppc += upgrade.pps }
            else { self.pps += upgrade.pps; }
            self.purchases[self.selection] += 1;
            self.infotext = format!("{} purchased!", upgrade.title)
        } else {
            self.infotext = format!("You can't afford a {}!", upgrade.title);
        }
        if self.selection == 0 && purchases == 0 {
            self.infotext = String::from("Great job! With enough pets, you might be able to upgrade your auto-petters and your own petting hand!\n(btw you can press '.' and ',' to turn the music up and down and esc, q, or ctrl-c at anytime will exit)\nHave fun petting Rock!")
        }
    }

    fn tick(&mut self) {
        self.pets += self.pps;
        self.tpps = self.pps + self.manualpets;
        self.manualpets = 0;
        if self.unlocked < NUMUPGRADES {
            if self.upgrades[self.unlocked].cost <= self.pets * 3 {
                self.unlock();
            }
        }
    }

    fn pet(&mut self) {
        if self.pets == 0 {
            self.infotext = String::from("Good job! Pet Rock a few more times, and you might be able to get an upgrade!")
        } else if self.pets > 3 && self.purchases[0] == 0 {
            self.infotext = String::from("You have enough for an upgrade! Click on it to select it, and use right click to buy it!\n(Or, use arrow keys to select and enter to buy)")
        }
        self.pets += self.ppc;
        self.manualpets += self.ppc;
        self.blushing = 15;
        if self.unlocked < NUMUPGRADES {
            if self.upgrades[self.unlocked].cost <= self.pets * 3 {
                self.unlocked += 1;
            }
        }
    }

    fn volume(&mut self, up: bool) {
        if up {
            self.volume += 0.1;
            if self.volume > 1. { self.volume = 1.; }
        } else {
            self.volume -= 0.1;
            if self.volume < 0. { self.volume = 0.; }
        }
        self.sink.set_volume(self.volume);
    }
}
