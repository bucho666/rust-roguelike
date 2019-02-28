extern crate termion;

use std::collections::HashMap;
use std::io::{stdin, stdout, Stdout, Write};
use std::ops::Add;
use std::ops::AddAssign;
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        *self = Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

struct Screen {
    screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            screen: AlternateScreen::from(stdout().into_raw_mode().unwrap()),
        }
    }

    fn goto(&mut self, coord: &Coord) {
        write!(
            self.screen,
            "{}",
            cursor::Goto(coord.x as u16 + 1, coord.y as u16 + 1)
        );
    }

    fn clear(&mut self) {
        write!(self.screen, "{}", clear::All);
        write!(self.screen, "{}", cursor::Goto(1, 1));
    }

    fn flush(&mut self) {
        self.screen.flush().unwrap();
    }

    fn write(&mut self, string: &str) {
        write!(self.screen, "{}", string);
    }
}

struct Walk {
    screen: Screen,
    cursor: Coord,
    key_map: HashMap<Key, Coord>,
    map: Vec<String>,
}

impl Walk {
    fn new() -> Walk {
        Walk {
            screen: Screen::new(),
            cursor: Coord { x: 1, y: 1 },
            key_map: [
                (Key::Char('j'), Coord { x: 0, y: 1 }),
                (Key::Char('k'), Coord { x: 0, y: -1 }),
                (Key::Char('l'), Coord { x: 1, y: 0 }),
                (Key::Char('h'), Coord { x: -1, y: 0 }),
                (Key::Char('y'), Coord { x: -1, y: -1 }),
                (Key::Char('u'), Coord { x: 1, y: -1 }),
                (Key::Char('b'), Coord { x: -1, y: 1 }),
                (Key::Char('n'), Coord { x: 1, y: 1 }),
            ]
                .iter()
                .cloned()
                .collect(),
            map: vec![
                String::from("###############################################"),
                String::from("#.............................................#"),
                String::from("#....########.###.............................#"),
                String::from("#....#.....#....#.............................#"),
                String::from("#....#.....#....#.............................#"),
                String::from("#....#.....#....#####.........................#"),
                String::from("#....#.....#....#....#........................#"),
                String::from("#....#######....#....#........................#"),
                String::from("#..........###########........................#"),
                String::from("#.............................................#"),
                String::from("###############################################"),
            ],
        }
    }

    fn run(&mut self) {
        self.draw();
        let stdin = stdin();
        for event in stdin.events() {
            self.process_event(event.unwrap());
        }
    }

    fn process_event(&mut self, event: Event) {
        match event {
            Event::Key(Key::Ctrl('c')) => {
                std::process::exit(0);
            }
            Event::Key(k) if self.key_map.contains_key(&k) => {
                self.move_me(self.key_map[&k]);
            }
            _ => {}
        }
    }

    fn draw(&mut self) {
        self.screen.clear();
        self.draw_map();
        self.draw_me();
        self.screen.flush();
    }

    fn draw_map(&mut self) {
        for line in &self.map {
            self.screen.write(&line);
            self.screen.write("\r\n");
        }
    }

    fn draw_me(&mut self) {
        self.screen.goto(&self.cursor);
        self.screen.write("@");
        self.screen.goto(&self.cursor);
    }

    fn move_me(&mut self, direction: Coord) {
        let next = self.cursor + direction;
        if self.can_walk(&next) {
            self.cursor = next;
            self.draw();
        }
    }

    fn can_walk(&mut self, coord: &Coord) -> bool {
        self.map[coord.y as usize]
            .chars()
            .nth(coord.x as usize)
            .unwrap()
            == '.'
    }
}

fn main() {
    Walk::new().run();
}
