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

pub const NUMUPGRADES: usize = 5;
pub fn upgrades() -> [Upgrade; NUMUPGRADES] {[
    Upgrade::new(false, '⌁', Color::White, "Auto-Petter", "Pets Rock for you (very slowly)", 5, 1, 1),
    Upgrade::new(true, '☁', Color::LightBlue, "Soft Glove", "Rock appreciates your pets more!", 25, 25, 1),
    Upgrade::new(false, '⌁', Color::LightYellow, "Better-Petter", "Pets Rock a little faster", 100, 50, 10),
    Upgrade::new(false, '⚙', Color::Green, "Pet-o-matic", "Pets Rock at its own discretion", 1000, 500, 100),
    Upgrade::new(true, '⑩', Color::Blue, "10 more arms", "more arm = more pet", 5000, 4000, 10)
]}
