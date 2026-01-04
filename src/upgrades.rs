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
}

pub const NUMUPGRADES: usize = 3;
pub fn upgrades() -> [Upgrade; NUMUPGRADES] {[
    Upgrade::new(false, '⌁', Color::White, "Auto-Petter", "Pets Rock for you (very slowly)", 5, 1, 1),
    Upgrade::new(true, '☁', Color::LightBlue, "Soft Glove", "Rock appreciates your pets more!", 20, 10, 1),
    Upgrade::new(false, '⌁', Color::LightYellow, "Better-Petter", "Pets Rock a little faster", 100, 50, 10),
    // Upgrade::new("Petter 4", "Pets at alarming speeds", 1000, 300, 50),
    // Upgrade::new("Petter 5", "Too much petting...", 8000, 1500, 500),
    // Upgrade::new("Petter 6", "PETTING OVERLOAD", 50000, 10000, 5000),
]}
