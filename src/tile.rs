use termion::color;

pub struct Tile {
    image: String,
}

impl Tile {
    pub fn new<C: color::Color>(tile: char, color: C) -> Tile {
        Tile {
            image: format!("{}{}", color::Fg(color), tile),
        }
    }

    pub fn image(&self) -> String {
        self.image.clone()
    }
}
