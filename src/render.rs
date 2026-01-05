use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, List, ListState, Paragraph}
};
use crate::{App, upgrades::Upgrade};

const ROCK: &str = "           ┌──────────┐
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

const EYES: [&str; 10] = [
    "
 ▄▄▄▄
█  ███
██████
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
██  ██
██████
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
███  █
██████
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
█▀▀███
█▄▄███
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
██▀▀██
██▄▄██
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
███▀▀█
███▄▄█
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
██████
█  ███
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
██████
██  ██
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
██████
███  █
 ▀▀▀▀
    ",
    "
 ▄▄▄▄
█▀  ▀█
██████
 ▀▀▀▀
    ",
];

const BLUSH: &str = "▒▒▒▒";

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let rounded_frame = Block::bordered().border_type(BorderType::Rounded);
        frame.render_widget(rounded_frame, frame.area());

        let vlayout = Layout::new(Direction::Vertical, [
            Constraint::Min(13),
            Constraint::Max(6), // height of the info text box
        ]).split(Rect::new(1, 1, frame.area().width - 2, frame.area().height - 2));
        let info_block = Paragraph::new(self.infotext.clone()).centered().block(Block::bordered().border_type(BorderType::Rounded));
        frame.render_widget(info_block, vlayout[1]);

        let hlayout = Layout::new(Direction::Horizontal, [
            Constraint::Fill(1),
            Constraint::Length(60), // width of the upgrades list
        ]).split(vlayout[0]);
        let (sidebar, mut sidebar_state) = self.render_sidebar();
        frame.render_stateful_widget(sidebar, hlayout[1], &mut sidebar_state);
        self.listoffset = sidebar_state.offset();

        self.render_rock(frame, hlayout[0]);
    }

    fn render_rock(&mut self, frame: &mut Frame, area: Rect) {
        self.render_background(frame, area);

        let rvlayout = Layout::new(Direction::Vertical, [
            Constraint::Fill(1),
            Constraint::Length(14),
            Constraint::Fill(1)
        ]).split(area);
        let rhlayout = Layout::new(Direction::Horizontal, [
            Constraint::Fill(1),
            Constraint::Length(36),
            Constraint::Fill(1)
        ]).split(rvlayout[1]);
        frame.render_widget(Text::raw(format!("Pets: {} | {}pps | {}ppc\nTotal pets per second: {}", ntostr(self.pets), ntostr(self.pps), ntostr(self.ppc), ntostr(self.tpps))).white().centered(), rhlayout[1]);
        frame.render_widget(Text::raw(ROCK).white(), Rect::new(rhlayout[1].x, rhlayout[1].y + 2, 36, 12));

        if self.blushing > 0 {
            let blushrect_a = Rect::new(rhlayout[1].x + 6, rhlayout[1].y + 9, 4, 1);
            let blushrect_b = Rect::new(rhlayout[1].x + 24, rhlayout[1].y + 9, 4, 1);
            frame.render_widget(Text::styled(BLUSH, Color::LightMagenta), blushrect_a);
            frame.render_widget(Text::styled(BLUSH, Color::LightMagenta), blushrect_b);
        }
        self.render_eye(frame, rhlayout[1].x + 9, rhlayout[1].y + 3);
        self.render_eye(frame, rhlayout[1].x + 19, rhlayout[1].y + 3);

        self.rockpos = Position::new(rhlayout[1].x + 19, rhlayout[1].y + 6);
    }

    fn render_eye(&self, frame: &mut Frame, x: u16, y: u16) {
        let rect = Rect::new(x, y, 16, 5);
        let deltax: isize = self.mousepos.x as isize - (x + 3) as isize;
        let deltay: isize = (y + 2) as isize - self.mousepos.y as isize;
        let mut eye: usize =
            if deltax.abs() < 4 && deltay.abs() < 3 { 4 }
            else if 2*deltax <= -deltay && deltax >= -2*deltay { 0 }
            else if deltay >= 2*deltax.abs() { 1 }
            else if 2*deltax >= deltay && deltax < 2*deltay { 2 }
            else if deltax <= -2*deltay.abs() { 3 }
            else if deltax >= 2*deltay.abs() { 5 }
            else if 2*deltax <= deltay && deltax >= 2*deltay { 6 }
            else if deltay < -2*deltax.abs() { 7 }
            else { 8 };
        if self.blushing > 0 { eye = 9; }
        frame.render_widget(Text::raw(EYES[eye]), rect);
    }

    fn render_background(&mut self, frame: &mut Frame, area: Rect) {
        let width = area.width as usize - 1;
        let height = area.height as usize;
        let mut bg: Vec<Vec<Span>> = vec![vec![Span::raw(" "); width]; height];
        for i in 0..self.unlocked {
            let mut rng = ChaCha8Rng::seed_from_u64(i as u64);
            for _ in 0..self.purchases[i] {
                let x = rng.random_range(0..width);
                let y = rng.random_range(0..height);
                bg[y][x] = Span::styled(self.upgrades[i].icon.0.clone(), self.upgrades[i].icon.1);
            }
        }
        let mut lines: Vec<Line> = Vec::new();
        for y in 0..height {
            lines.push(Line::from(bg[y].clone()));
        }
        frame.render_widget(Text::from(lines), area);
    }

    fn render_sidebar(&self) -> (List<'_>, ListState) {
        let mut state = ListState::default().with_offset(self.listoffset);
        state.select(Some(self.selection));
        let mut upgradetexts: Vec<Text> = Vec::new();
        for i in 0..self.unlocked {
            let upgradetext = self.upgrades[i].render(self.purchases[i]);
            if self.pets < self.upgrades[i].cost(self.purchases[i])  {
                upgradetexts.push(upgradetext.style(Color::DarkGray));
            } else {
                upgradetexts.push(upgradetext);
            }
        }
        let list = List::new(upgradetexts)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .highlight_style(Style::new().bold()).highlight_symbol("> ");
        return (list, state);
    }
}

impl Upgrade {
    pub fn render(&self, purchases: usize) -> Text<'_> {
        let mut text = Text::raw(format!("{}: ", self.title));
        // text.push_span(Span::styled(self.icon.0.clone(), self.icon.1));
        text.push_span(Span::raw(self.description.clone()));
        if self.hand {
            text.push_line(format!("Costs {} pets, yields {} more pets per click", ntostr(self.cost(purchases)), ntostr(self.pps)));
            text.push_line(Line::styled(format!("{} purchased - {} pets per ppc", purchases, ntostr(self.cost(purchases) / self.pps)), Style::new().green().dim()));
        }
        else {
            text.push_line(format!("Costs {} pets, yields {} more pets per second", ntostr(self.cost(purchases)), ntostr(self.pps)));
            text.push_line(Line::styled(format!("{} purchased - {} pets per pps", purchases, ntostr(self.cost(purchases) / self.pps)), Style::new().blue().dim()));
        }
        text.push_line(Line::raw(""));
        return text;
    }
}

const SUFFIXES: [char; 5] = ['k', 'M', 'G', 'P', 'E'];

fn ntostr(input: usize) -> String {
    let mut n: f64 = input as f64;
    if n < 10000. { return format!("{}", n); }
    n /= 1000.;
    let mut i = 0;
    while n > 999.9 {
        n /= 1000.;
        i += 1;
    }
    return format!("{:.1}{}", n, SUFFIXES[i]);
}
