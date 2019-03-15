#[macro_use]
extern crate lazy_static;
extern crate termion;
mod coord;
mod map;
mod screen;
mod terrain;
use crate::coord::Coord;
use crate::map::Map;
use crate::screen::Screen;
use std::collections::HashMap;
use std::io::stdin;
use termion::event::{Event, Key};
use termion::input::TermRead;

struct Walk {
    screen: Screen,
    cursor: Coord,
    key_map: HashMap<Key, Coord>,
    map: Map,
}

enum WalkState {
    Continue,
    Quit,
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
            map: Map::new(vec![
                "########################",
                "#......................#",
                "#....########.###......#",
                "#....#..........#......#",
                "#....#.xxx.#....#......#",
                "#....#.xxx.#....#####..#",
                "#....#.....#....#....#.#",
                "#....##.####.........#.#",
                "#...........##########.#",
                "#......................#",
                "########################",
            ]),
        }
    }

    fn run(&mut self) {
        self.draw();
        let stdin = stdin();
        for event in stdin.events() {
            if let WalkState::Quit = self.process_event(event.unwrap()) {
                return;
            }
        }
    }

    fn process_event(&mut self, event: Event) -> WalkState {
        match event {
            Event::Key(Key::Ctrl('c')) => {
                return WalkState::Quit;
            }
            Event::Key(k) if self.key_map.contains_key(&k) => {
                self.move_me(self.key_map[&k]);
            }
            _ => {}
        }
        return WalkState::Continue;
    }

    fn draw(&mut self) {
        self.screen.clear();
        self.draw_map();
        self.draw_me();
        self.screen.flush();
    }

    fn draw_map(&mut self) {
        self.screen.write(&self.map.image());
    }

    fn draw_me(&mut self) {
        self.screen.goto(&self.cursor);
        self.screen.write("@");
        self.screen.goto(&self.cursor);
    }

    fn move_me(&mut self, direction: Coord) {
        let next = self.cursor + direction;
        if self.map.can_walk(&next) {
            self.cursor = next;
            self.draw();
        }
    }
}

fn main() {
    Walk::new().run();
}
