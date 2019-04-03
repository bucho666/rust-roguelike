#[macro_use]
extern crate lazy_static;
extern crate termion;
mod coord;
mod entity;
mod map;
mod screen;
mod terrain;
mod tile;
use crate::coord::Coord;
use crate::entity::{EntityId, EntitySystem};
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
}

impl Object {
    pub fn new<C: color::Color>(tile: char, color: C) -> Object {
        Object {
            tile: Tile::new(tile, color),
        }
    }

    pub fn image(&self) -> String {
        self.tile.image()
    }
}

struct Walk {
    screen: Screen,
    player: EntityId,
    key_map: HashMap<Key, Coord>,
    map: Map,
    entity: EntitySystem,
}

enum WalkState {
    Continue,
    Quit,
}

impl Walk {
    fn new() -> Walk {
        let mut ecs = EntitySystem::new();
        let player = ecs.register(Object::new('@', color::White));
        Walk {
            screen: Screen::new(),
            player: player,
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
            entity: ecs,
        }
    }

    fn run(&mut self) {
        self.map.add_entity(self.player, Coord::new(1, 1));
        for i in 0..3 {
            self.map.add_entity(
                self.entity.register(Object::new('o', color::Green)),
                Coord::new(3, 6 + i),
            );
        }
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
        for (coord, id) in self.map.entities() {
            let image = self.object(id).image();
            self.screen.goto(&coord);
            self.screen.write(&image);
        }
    }

    fn draw_player(&mut self) {
        self.screen.goto(&self.map.coord_of(self.player));
    }

    fn object(&mut self, id: EntityId) -> &mut Object {
        self.entity.of_mut::<Object>(id)
    }

    fn move_player(&mut self, direction: Coord) {
        let to = self.map.coord_of(self.player) + direction;
        if self.map.can_walk(to) {
            self.map.move_entity(self.player, to);
            self.draw();
        }
    }
}

fn main() {
    Walk::new().run();
}
