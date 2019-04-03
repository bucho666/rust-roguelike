#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate termion;
mod coord;
mod direction;
mod entity;
mod map;
mod screen;
mod terrain;
mod tile;
use crate::coord::Coord;
use crate::direction::*;
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
                (Key::Char('j'), S),
                (Key::Char('k'), N),
                (Key::Char('l'), E),
                (Key::Char('h'), W),
                (Key::Char('y'), NW),
                (Key::Char('u'), NE),
                (Key::Char('b'), SW),
                (Key::Char('n'), SE),
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
                self.move_monsters();
                self.draw();
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
        self.move_entity(self.player, direction);
    }

    fn move_entity(&mut self, entity: EntityId, direction: Coord) {
        let to = self.map.coord_of(entity) + direction;
        if self.map.can_walk(to) {
            self.map.move_entity(entity, to);
        }
    }

    fn move_monsters(&mut self) {
        for (_, id) in self.map.entities() {
            if id != self.player {
                self.move_entity(id, random_direction());
            }
        }
    }
}

fn main() {
    Walk::new().run();
}
