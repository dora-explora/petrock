use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListState, Paragraph}
};
use crate::{App, upgrades::Upgrade};

const ROCK: &str = "
           ┌──────────┐
       ┌───┘          └──┐
     ┌─┘                 └─┐
   ┌─┘                     └─┐
  ┌┘                         └─┐
 ┌┘                            └─┐
┌┘                               └┐
│                                 └┐
└┐                                ┌┘
 └─┐                            ┌─┘
   └────────────────────────────┘
";

const EYES: [&str; 9] = [
    "
 ▄▄▄▄      ▄▄▄▄
█  ███    █  ███
██████    ██████
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
██  ██    ██  ██
██████    ██████
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
███  █    ███  █
██████    ██████
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
█▀▀███    █▀▀███
█▄▄███    █▄▄███
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
██▀▀██    ██▀▀██
██▄▄██    ██▄▄██
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
███▀▀█    ███▀▀█
███▄▄█    ███▄▄█
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
██████    ██████
█  ███    █  ███
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
██████    ██████
██  ██    ██  ██
 ▀▀▀▀      ▀▀▀▀
    ",
    "
 ▄▄▄▄      ▄▄▄▄
██████    ██████
███  █    ███  █
 ▀▀▀▀      ▀▀▀▀
    ",
];

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let rounded_frame = Block::bordered().border_type(BorderType::Rounded);
        frame.render_widget(rounded_frame, frame.area());

        let vlayout = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(6), // height of the info text box
        ]).split(Rect::new(1, 1, frame.area().width - 2, frame.area().height - 2));
        let info_block = Paragraph::new(self.infotext.clone()).centered().block(Block::bordered().border_type(BorderType::Rounded));
        frame.render_widget(info_block, vlayout[1]);

        let hlayout = Layout::new(Direction::Horizontal, [
            Constraint::Fill(1),
            Constraint::Length(50), // width of the upgrades list
        ]).split(vlayout[0]);
        let (sidebar, mut sidebar_state) = self.render_sidebar();
        frame.render_stateful_widget(sidebar, hlayout[1], &mut sidebar_state);
        self.listoffset = sidebar_state.offset();

        frame.render_widget(Text::raw(format!("Pets: {}\n{} pets per second", ntostr(self.pets), ntostr(self.pps))), hlayout[0]);

        let rvlayout = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(12),
            Constraint::Fill(1)
        ]).split(hlayout[0]);
        let rhlayout = Layout::new(Direction::Horizontal, [
            Constraint::Fill(1),
            Constraint::Length(36),
            Constraint::Fill(1)
        ]).split(rvlayout[1]);
        frame.render_widget(Text::raw(ROCK), rhlayout[1]);

        let eyerect = Rect::new(rhlayout[1].x + 9, rhlayout[1].y + 3, 16, 5);
        let deltax: isize = self.mousepos.x as isize - (rhlayout[1].x + 17) as isize;
        let deltay: isize = (rhlayout[1].y + 5) as isize - self.mousepos.y as isize;
        let eye: usize =
            if deltax*deltax/5 + deltay*deltay < 40 { 4 }
            else if 2*deltax <= -deltay && deltax >= -2*deltay { 0 }
            else if deltay >= 2*deltax.abs() { 1 }
            else if 2*deltax >= deltay && deltax < 2*deltay { 2 }
            else if deltax <= -2*deltay.abs() { 3 }
            else if deltax >= 2*deltay.abs() { 5 }
            else if 2*deltax <= deltay && deltax >= 2*deltay { 6 }
            else if deltay < -2*deltax.abs() { 7 }
            else { 8 };
        frame.render_widget(Text::raw(EYES[eye]), eyerect);

        self.rockpos = Position::new(rhlayout[1].x + 19, rhlayout[1].y + 6);
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
    while n > 999.9 {
        n /= 1000.;
        i += 1;
    }
    if (i < 5) { return format!("{:.1}{}", n, SUFFIXES[i]); }
    else { return format!("{:.1}E{}", n, i * 3); }
}
