use crate::coord::Coord;
use std::io::{stdout, Stdout, Write};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

pub struct Screen {
    screen: AlternateScreen<RawTerminal<Stdout>>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            screen: AlternateScreen::from(stdout().into_raw_mode().unwrap()),
        }
    }

    pub fn goto(&mut self, coord: Coord) {
        write!(
            self.screen,
            "{}",
            cursor::Goto(coord.x as u16 + 1, coord.y as u16 + 1)
        )
        .unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.screen, "{}", clear::All).unwrap();
        write!(self.screen, "{}", cursor::Goto(1, 1)).unwrap();
    }

    pub fn flush(&mut self) {
        self.screen.flush().unwrap();
    }

    pub fn write(&mut self, string: &str) {
        write!(self.screen, "{}", string).unwrap();
    }

    pub fn reset(&mut self) {
        write!(self.screen, "{}", color::Fg(color::Reset)).unwrap();
        self.clear();
        self.flush();
    }
}
