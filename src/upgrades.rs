use std::str::FromStr;

#[derive(Clone)]
pub struct Upgrade {
    pub title: String,
    pub description: String,
    pub cost: usize,
    pub factor: usize, // coefficient of purchase scaling factor
    pub pps: usize, // pets per minute, of course
}

impl Upgrade {
    fn new(title: &str, description: &str, cost: usize, factor: usize, pps: usize) -> Upgrade {
        return Upgrade {
            title: String::from_str(title).unwrap(),
            description: String::from_str(description).unwrap(),
            cost,
            factor,
            pps,
        };
    }
}

pub const NUMUPGRADES: usize = 6;
pub fn upgrades() -> [Upgrade; NUMUPGRADES] {[
    Upgrade::new("Petter 1", "Pets very slowly", 5, 1, 1),
    Upgrade::new("Petter 2", "Pets a little faster", 50, 10, 5),
    Upgrade::new("Petter 3", "Pets pretty fast!", 200, 50, 20),
    Upgrade::new("Petter 4", "Pets at alarming speeds", 1000, 200, 100),
    Upgrade::new("Petter 5", "Too much petting...", 5000, 1000, 500),
    Upgrade::new("Petter 6", "PETTING OVERLOAD", 10000, 5000, 5000),
]}
