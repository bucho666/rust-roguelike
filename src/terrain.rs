use crate::tile::Tile;
use std::collections::HashMap;
use termion::color;

pub struct Terrain {
    tile: Tile,
    can_walk: bool,
}

impl Terrain {
    pub fn new<C: color::Color>(tile: char, color: C, can_walk: bool) -> Terrain {
        Terrain {
            tile: Tile::new(tile, color),
            can_walk: can_walk,
        }
    }

    pub fn image(&self) -> String {
        self.tile.image()
    }

    pub fn can_walk(&self) -> bool {
        self.can_walk
    }
}

lazy_static! {
    pub static ref TERRAIN: HashMap<&'static str, Terrain> = {
        let mut t = HashMap::new();
        t.insert("null", Terrain::new(' ', color::Reset, false));
        t.insert("floor", Terrain::new('.', color::Green, true));
        t.insert("wall", Terrain::new('#', color::Reset, false));
        t
    };
}

impl TERRAIN {
    pub fn of(&self, name: &'static str) -> &Terrain {
        self.get(name).unwrap()
    }
}
