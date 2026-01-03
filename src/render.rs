use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListState, Paragraph}
};
use crate::{App, upgrades::Upgrade};

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let rounded_frame = Block::bordered().border_type(BorderType::Rounded);
        frame.render_widget(rounded_frame, frame.area());

        let vlayout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(6), // height of the info text box
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
        self.listoffset = sidebar_state.offset();

        frame.render_widget(Text::raw(format!("Pets: {}\n{} pets per second", ntostr(self.pets), ntostr(self.pps))), hlayout[0]);
    }

    fn render_sidebar(&self) -> (List, ListState) {
        let mut state = ListState::default().with_offset(self.listoffset);
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
        let cost = self.cost + self.factor * (purchases * (purchases + 1)) / 2;
        let topstring: String = format!("{}: {}\nCosts {} pets, yields {} pets per second", self.title, self.description, ntostr(cost), ntostr(self.pps));
        let mut text = Text::raw(topstring);
        text.push_line(Line::styled(format!("{} purchased", purchases), Style::new().dark_gray()));
        text.push_line(Line::raw(""));
        return text;
    }
}

const SUFFIXES: [char; 5] = ['K', 'M', 'B', 'T', 'Q'];

fn ntostr(input: usize) -> String {
    let mut n: f64 = input as f64;
    if n < 10000. { return format!("{}", n); }
    n /= 1000.;
    let mut i = 0;
    while n < 10. {
        n /= 1000.;
        i += 1;
    }
    return format!("{:.1}{}", n, SUFFIXES[i]);
}
