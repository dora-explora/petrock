use std::str::FromStr;
use ratatui::text::Text;

pub struct Upgrade {
    pub title: String,
    pub description: String,
    pub cost: usize,
    pub ppm: usize, // pets per minute, of course
}

impl Upgrade {
    fn new(title: &str, description: &str, cost: usize, ppm: usize) -> Upgrade {
        return Upgrade {
            title: String::from_str(title).unwrap(),
            description: String::from_str(description).unwrap(),
            cost,
            ppm,
        };
    }
}

pub const NUMUPGRADES: usize = 6;
pub fn upgrades() -> [Upgrade; NUMUPGRADES] {[
    Upgrade::new("Petter 1", "Pets very slowly", 5, 2),
    Upgrade::new("Petter 2", "Pets a little faster", 20, 10),
    Upgrade::new("Petter 3", "Pets every second!", 60, 40),
    Upgrade::new("Petter 4", "Pets at alarming speeds", 200, 200),
    Upgrade::new("Petter 5", "Too much petting...", 1000, 1500),
    Upgrade::new("Petter 6", "PETTING OVERLOAD", 10000, 5000),
]}
