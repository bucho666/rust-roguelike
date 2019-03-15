use crate::coord::Coord;
use crate::terrain::Terrain;
use crate::terrain::TERRAIN;

pub struct Map {
    map: Vec<Vec<&'static Terrain>>,
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
        Map { map: map_data }
    }

    pub fn image(&self) -> String {
        let mut data = String::from("");
        for line in &self.map {
            for &t in line {
                data += &t.tile.image;
            }
            data += "\r\n";
        }
        data
    }

    pub fn can_walk(&mut self, coord: &Coord) -> bool {
        self.map[coord.y as usize][coord.x as usize].can_walk
    }
}
