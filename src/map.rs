use crate::coord::Coord;
use crate::entity::EntityId;
use crate::terrain::{Terrain, TERRAIN};
use std::collections::HashMap;
use std::hash::Hash;

pub struct MutualHashMap<K, V> {
    vmap: HashMap<K, V>,
    kmap: HashMap<V, K>,
}

impl<K: Hash + Eq + Copy + Clone, V: Hash + Eq + Copy + Clone> MutualHashMap<K, V> {
    pub fn new() -> Self {
        MutualHashMap {
            vmap: HashMap::new(),
            kmap: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.vmap.insert(key, value);
        self.kmap.insert(value, key);
    }

    pub fn value(&self, key: K) -> Option<&V> {
        self.vmap.get(&key)
    }

    pub fn key(&self, key: V) -> K {
        self.kmap[&key]
    }

    //pub fn contains_key(&self, key: K) -> bool {
    //    self.vmap.contains_key(&key)
    //}

    //pub fn values(&self) -> Vec<V> {
    //    self.vmap.values().map(|v| *v).collect()
    //}

    pub fn entries(&self) -> Vec<(K, V)> {
        self.vmap.iter().map(|(k, v)| (*k, *v)).collect()
    }

    //pub fn remove_by_key(&mut self, key: K) {
    //    self.kmap.remove(&self.value(key));
    //    self.vmap.remove(&key);
    //}

    pub fn remove_by_value(&mut self, value: V) {
        self.vmap.remove(&self.key(value));
        self.kmap.remove(&value);
    }
}

pub struct Map {
    map: Vec<Vec<&'static Terrain>>,
    character: MutualHashMap<Coord, EntityId>,
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
            character: MutualHashMap::new(),
        }
    }

    pub fn remove_character(&mut self, id: EntityId) {
        self.character.remove_by_value(id);
    }

    pub fn move_character(&mut self, id: EntityId, to: Coord) {
        self.remove_character(id);
        self.character.insert(to, id);
    }

    pub fn add_character(&mut self, id: EntityId, coord: Coord) {
        self.character.insert(coord, id);
    }

    pub fn character_at(&self, coord: Coord) -> Option<EntityId> {
        if let Some(e) = self.character.value(coord) {
            return Some(*e);
        }
        None
    }

    pub fn characters(&self) -> Vec<(Coord, EntityId)> {
        self.character.entries()
    }

    pub fn coord_of(&self, id: EntityId) -> Coord {
        self.character.key(id)
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

    pub fn terrain_at(&self, coord: Coord) -> &Terrain {
        self.map[coord.y as usize][coord.x as usize]
    }

    pub fn can_walk(&mut self, coord: Coord) -> bool {
        self.terrain_at(coord).can_walk()
    }
}
