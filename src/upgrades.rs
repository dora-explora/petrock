use std::str::FromStr;
use ratatui::style::Color;

#[derive(Clone)]
pub struct Upgrade {
    pub hand: bool, // whether or not this upgrade applies to clicks
    pub icon: (String, Color), // one character and color that represents item
    pub title: String,
    pub description: String,
    pub cost: usize,
    pub factor: usize, // coefficient of purchase scaling factor
    pub pps: usize, // pets per second/click
}

impl Upgrade {
    fn new(hand: bool, iconchar: char, iconcolor: Color, title: &str, description: &str, cost: usize, factor: usize, pps: usize) -> Upgrade {
        return Upgrade {
            hand,
            icon: (String::from(iconchar), iconcolor),
            title: String::from_str(title).unwrap(),
            description: String::from_str(description).unwrap(),
            cost,
            factor,
            pps,
        };
    }

    pub fn cost(&self, purchases: usize) -> usize {
        return self.cost + self.factor * (purchases * (purchases + 1)) / 2; // thank you math class
    }
}

pub const NUMUPGRADES: usize = 12;
pub fn upgrades() -> [Upgrade; NUMUPGRADES] {[
    Upgrade::new(false, '⌁', Color::White, "Auto-Petter", "Pets Rock for you (very slowly)",     5, 1, 1),
    Upgrade::new(true, '☁', Color::LightBlue, "Soft Glove", "Rock appreciates your pets more!", 25, 25, 1),
    Upgrade::new(false, '⌁', Color::LightYellow, "Better-Petter", "Pets Rock a little faster",   100, 50, 10),
    Upgrade::new(true, '🌡', Color::LightRed, "Hand Warmers", "Comfier for you and for Rock",    500, 350, 5),
    Upgrade::new(false, '⚙', Color::Green, "Pet-o-matic", "Pets Rock at its own discretion",    3000, 1500, 50),
    Upgrade::new(true, '🙌', Color::Blue, "25 more arms", "more arm = more pet",                 5000, 3500, 25),
    Upgrade::new(false, '⌁', Color::LightGreen, "Better-Petter-o-matic", "the companies merged", 15000, 5000, 200),
    Upgrade::new(false, '!', Color::LightRed, "Petter Overclock", "1000 Hz auto-petter",         100000, 50000, 1000),
    Upgrade::new(true, '💪', Color::Yellow, "Gym Sesh", "No pain, no gain",                      250000, 150000, 250),
    Upgrade::new(false, '⚛', Color::Cyan, "Quantum Petter", "Superposition of 5000 pets",        1000000, 500000, 5000),
    Upgrade::new(true, '🦾', Color::White, "Cyborg Enhancements", "You're a petting machine!",   2500000, 2000000, 2500),
    Upgrade::new(false, '⌨', Color::Magenta, "Petting Machine", "nvm",                          15000000, 7500000, 50000),
]}
