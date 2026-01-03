use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, List, ListState, Paragraph}
};
use crate::{App, upgrades::Upgrade};

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let rounded_frame = Block::bordered().border_type(BorderType::Rounded);
        frame.render_widget(rounded_frame, frame.area());

        let vlayout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(3),
                Constraint::Fill(1),
            ]).split(Rect::new(1, 1, frame.area().width - 2, frame.area().height - 2));
        let info_block = Paragraph::new(self.infotext.clone()).centered().block(Block::bordered().border_type(BorderType::Rounded));
        frame.render_widget(info_block, vlayout[1]);

        let hlayout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(50), // width of the upgrades list

            ]).split(vlayout[0]);
        let (sidebar, mut sidebar_state) = self.render_sidebar();
        frame.render_stateful_widget(sidebar, hlayout[1], &mut sidebar_state);
    }

    fn render_sidebar(&self) -> (List, ListState) {
        let mut state = ListState::default();
        state.select(Some(self.selection));
        let mut upgradetexts: Vec<Text> = Vec::new();
        for i in 0..self.unlocked {
            upgradetexts.push(self.upgrades[i].render(self.purchases[i]));
        }
        let list = List::new(upgradetexts)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .highlight_style(Style::new().fg(Color::Yellow));
        return (list, state);
    }
}

impl Upgrade {
    pub fn render(&self, purchases: usize) -> Text {
        let string: String = format!("{}: {}\nCosts {} pets, yields {} pets per minute\n ", self.title, self.description, ntostr(self.cost << purchases), ntostr(self.ppm));
        return Text::raw(string);
    }
}

const SUFFIXES: [char; 5] = ['K', 'M', 'B', 'T', 'Q'];

fn ntostr(input: usize) -> String {
    let mut n = input;
    if n < 10000 { return format!("{}", n); }
    n /= 1000;
    let mut i = 0;
    while (n < 10) {
        n /= 1000;
        i += 1;
    }
    return format!("{}{}", n, SUFFIXES[i]);
}
