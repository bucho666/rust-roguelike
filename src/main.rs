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
use std::any::Any;
use std::collections::HashMap;
use std::io::stdin;
use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;

struct Character {
    tile: Tile,
    name: String,
}

impl Character {
    pub fn new<C: color::Color>(tile: char, name: &str, color: C) -> Character {
        Character {
            tile: Tile::new(tile, color),
            name: name.to_string(),
        }
    }

    pub fn image(&self) -> String {
        self.tile.image()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

enum MoveResult {
    Moved,
    Blocked,
    BlockCharacter(EntityId),
}

struct World {
    entities: EntitySystem,
}

impl World {
    fn new() -> Self {
        World {
            entities: EntitySystem::new(),
        }
    }

    fn create_entity<T: Any>(&mut self, entity: T) -> EntityId {
        self.entities.register(entity)
    }

    fn character(&mut self, id: EntityId) -> &mut Character {
        self.entities.of_mut::<Character>(id)
    }

    fn map(&mut self, id: EntityId) -> &mut Map {
        self.entities.of_mut::<Map>(id)
    }

    fn put_character(&mut self, ch: EntityId, map_id: EntityId, coord: Coord) {
        self.map(map_id).add_character(ch, coord);
    }

    fn move_character(&mut self, ch: EntityId, map_id: EntityId, direction: Coord) -> MoveResult {
        let to = self.map(map_id).coord_of(ch) + direction;
        if let Some(ch) = self.map(map_id).character_at(to) {
            return MoveResult::BlockCharacter(ch);
        }
        if !self.map(map_id).can_walk(to) {
            return MoveResult::Blocked;
        }
        self.map(map_id).move_character(ch, to);
        MoveResult::Moved
    }

    fn kill(&mut self, ch: EntityId, map: EntityId) {
        self.map(map).remove_character(ch);
        self.entities.remove(ch);
    }
}

enum WalkState {
    Continue,
    Quit,
}

struct Walk {
    screen: Screen,
    move_key: HashMap<Key, Coord>,
    player: EntityId,
    map: EntityId,
    world: World,
    message: String,
}

impl Walk {
    fn new() -> Walk {
        let mut world = World::new();
        Walk {
            screen: Screen::new(),
            player: world.create_entity(Character::new('@', "hero", color::White)),
            map: world.create_entity(Map::new(vec![
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
            ])),
            move_key: [
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
            world: world,
            message: String::new(),
        }
    }

    fn run(&mut self) {
        self.init();
        self.draw();
        let stdin = stdin();
        for event in stdin.events() {
            if let WalkState::Quit = self.process_event(event.unwrap()) {
                break;
            }
        }
        self.screen.reset();
    }

    fn init(&mut self) {
        let (player, map) = (self.player, self.map);
        self.world.put_character(player, map, Coord::new(1, 1));
        for i in 0..3 {
            let orc = Character::new('o', "orc", color::Green);
            let orc_id = self.world.create_entity(orc);
            self.world.put_character(orc_id, map, Coord::new(3, 6 + i));
        }
        let pname = self.world.character(self.player).name();
        self.push_message(&format!("welcome {}", pname), color::Reset);
    }

    fn process_event(&mut self, event: Event) -> WalkState {
        match event {
            Event::Key(Key::Ctrl('c')) => {
                return WalkState::Quit;
            }
            Event::Key(k) if self.move_key.contains_key(&k) => {
                self.move_player(self.move_key[&k]);
            }
            _ => {}
        }
        WalkState::Continue
    }

    fn next_turn(&mut self) {
        self.move_monsters();
        self.draw();
    }

    fn draw(&mut self) {
        self.screen.clear();
        self.draw_map();
        self.draw_characters();
        self.draw_message();
        self.move_cursor_to_player();
        self.screen.flush();
    }

    fn push_message<C: color::Color>(&mut self, message: &str, color: C) {
        let new_message = format!("{}{}", color::Fg(color), message);
        if !self.message.is_empty() {
            self.message += "\r\n";
        }
        self.message += &new_message;
    }

    fn draw_message(&mut self) {
        if self.message.is_empty() {
            return;
        }
        self.screen.goto(Coord::new(0, 11));
        self.screen.write(&self.message);
        self.message.clear();
    }

    fn draw_map(&mut self) {
        let image = self.map().image();
        self.screen.write(&image);
    }

    fn draw_characters(&mut self) {
        for (coord, id) in self.map().characters() {
            let image = self.character(id).image();
            self.screen.goto(coord);
            self.screen.write(&image);
        }
    }

    fn move_cursor_to_player(&mut self) {
        let player = self.player;
        let coord = self.map().coord_of(player);
        self.screen.goto(coord);
    }

    fn move_player(&mut self, direction: Coord) {
        match self.move_character(self.player, direction) {
            MoveResult::BlockCharacter(m) => {
                let name = self.world.character(m).name();
                self.push_message(&format!("kill {}", name), color::Red);
                self.world.kill(m, self.map);
            }
            MoveResult::Blocked => {
                return;
            }
            _ => {}
        }
        self.next_turn();
    }

    fn move_monsters(&mut self) {
        for (_, id) in self.map().characters() {
            if id != self.player {
                self.move_character(id, random_direction());
            }
        }
    }

    fn move_character(&mut self, entity: EntityId, direction: Coord) -> MoveResult {
        self.world.move_character(entity, self.map, direction)
    }

    fn character(&mut self, id: EntityId) -> &mut Character {
        self.world.character(id)
    }

    fn map(&mut self) -> &mut Map {
        self.world.map(self.map)
    }
}

fn main() {
    Walk::new().run();
}
