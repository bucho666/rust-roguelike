use crate::coord::Coord;
use crate::entity::EntityId;
use crate::terrain::{Terrain, TERRAIN};
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

pub struct Map {
    map: Vec<Vec<&'static Terrain>>,
    entities: HashMap<TypeId, HashSet<EntityId>>,
}

impl Map {
    pub fn new(design: Vec<&'static str>) -> Map {
        let mut map_data: Vec<Vec<&'static Terrain>> = vec![];
        for line in design {
            let mut terrain_line: Vec<&'static Terrain> = vec![];
            for ch in line.chars() {
                let t = match ch {
                    '.' => TERRAIN.of("floor"),
                    '#' => TERRAIN.of("wall"),
                    _ => TERRAIN.of("null"),
                };
                terrain_line.push(t);
            }
            map_data.push(terrain_line);
        }
        Map {
            map: map_data,
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, id: EntityId) {
        let type_id = id.type_id();
        if !self.entities.contains_key(&type_id) {
            self.entities.insert(type_id, HashSet::new());
        }
        self.entities.get_mut(&type_id).unwrap().insert(id);
    }

    pub fn entities<T: Any>(&self) -> Vec<EntityId> {
        self.entities[&TypeId::of::<T>()]
            .clone()
            .into_iter()
            .collect()
    }

    pub fn image(&self) -> String {
        let mut data = String::from("");
        for line in &self.map {
            for &t in line {
                data += &t.image();
            }
            data += "\r\n";
        }
        data
    }

    pub fn can_walk(&mut self, coord: &Coord) -> bool {
        self.map[coord.y as usize][coord.x as usize].can_walk()
    }
}
