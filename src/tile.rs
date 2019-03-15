use termion::color;

pub struct Tile {
    pub image: String,
}

impl Tile {
    pub fn new<C: color::Color>(tile: char, color: C) -> Tile {
        Tile {
            image: format!("{}{}", color::Fg(color), tile),
        }
    }
}
