#[macro_use]
extern crate lazy_static;
extern crate termion;
mod coord;
mod map;
mod screen;
mod terrain;
mod tile;
use crate::coord::Coord;
use crate::map::Map;
use crate::screen::Screen;
use crate::tile::Tile;
use std::collections::HashMap;
use std::io::stdin;
use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;

struct Object {
    tile: Tile,
    coord: Coord,
}

impl Object {
    pub fn new<C: color::Color>(tile: char, color: C) -> Object {
        Object {
            tile: Tile::new(tile, color),
            coord: Coord::new(0, 0),
        }
    }
}

struct Walk {
    screen: Screen,
    player: Object,
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
            player: Object::new('@', color::White),
            key_map: [
                (Key::Char('j'), Coord::new(0, 1)),
                (Key::Char('k'), Coord::new(0, -1)),
                (Key::Char('l'), Coord::new(1, 0)),
                (Key::Char('h'), Coord::new(-1, 0)),
                (Key::Char('y'), Coord::new(-1, -1)),
                (Key::Char('u'), Coord::new(1, -1)),
                (Key::Char('b'), Coord::new(-1, 1)),
                (Key::Char('n'), Coord::new(1, 1)),
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
        self.player.coord = Coord::new(1, 1);
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
                self.move_player(self.key_map[&k]);
            }
            _ => {}
        }
        return WalkState::Continue;
    }

    fn draw(&mut self) {
        self.screen.clear();
        self.draw_map();
        self.draw_player();
        self.screen.flush();
    }

    fn draw_map(&mut self) {
        self.screen.write(&self.map.image());
    }

    fn draw_player(&mut self) {
        self.screen.goto(&self.player.coord);
        self.screen.write(&self.player.tile.image);
        self.screen.goto(&self.player.coord);
    }

    fn move_player(&mut self, direction: Coord) {
        let next = self.player.coord + direction;
        if self.map.can_walk(&next) {
            self.player.coord = next;
            self.draw();
        }
    }
}

fn main() {
    Walk::new().run();
}
