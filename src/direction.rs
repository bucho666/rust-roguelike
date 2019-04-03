use crate::coord::Coord;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub static N: Coord = Coord { x: 0, y: -1 };
pub static E: Coord = Coord { x: 1, y: 0 };
pub static S: Coord = Coord { x: 0, y: 1 };
pub static W: Coord = Coord { x: -1, y: 0 };
pub static NE: Coord = Coord { x: 1, y: -1 };
pub static SE: Coord = Coord { x: 1, y: 1 };
pub static NW: Coord = Coord { x: -1, y: -1 };
pub static SW: Coord = Coord { x: -1, y: 1 };
pub static DIRECTIONS: [Coord; 8] = [N, E, S, W, NE, SE, SW, NW];

pub fn random_direction() -> Coord {
    *DIRECTIONS.choose(&mut thread_rng()).unwrap()
}
