use rand::Rng;
use regex::Regex;

pub struct Dice {
    value: i16,
    side: i16,
    bonus: i16,
}

impl Dice {
    pub fn new(text: &str) -> Self {
        lazy_static! {
            static ref DICE_RE: Regex = Regex::new(r"^(\d+)[dD](\d+)").unwrap();
            static ref BONUS_RE: Regex = Regex::new(r"([+-]\d+)$").unwrap();
        }
        let dice_cap = DICE_RE.captures(text).unwrap();
        Dice {
            value: *&dice_cap[1].parse::<i16>().unwrap(),
            side: *&dice_cap[2].parse::<i16>().unwrap(),
            bonus: match BONUS_RE.captures(text) {
                None => 0,
                Some(bonus_cap) => *&bonus_cap[1].parse::<i16>().unwrap(),
            },
        }
    }

    pub fn roll(&self) -> i16 {
        let mut roll = 0;
        for _ in 0..self.value {
            roll += rand::thread_rng().gen_range(1, self.side);
        }
        roll + self.bonus
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_test() {
        let mut d = Dice::new("1d6");
        assert_eq!((d.value, d.side), (1, 6));
        assert_eq!(d.bonus, 0);
        d = Dice::new("2d16+3");
        assert_eq!((d.value, d.side), (2, 16));
        assert_eq!(d.bonus, 3);
        d = Dice::new("3d4-5");
        assert_eq!((d.value, d.side), (3, 4));
        assert_eq!(d.bonus, -5);
    }

    #[test]
    fn roll_test() {
        let mut d = Dice::new("1d6");
        for _ in 1..100 {
            let roll = d.roll();
            assert!(roll >= 1 && roll <= 6)
        }
        d = Dice::new("2d6+2");
        for _ in 1..100 {
            let roll = d.roll();
            assert!(roll >= 4 && roll <= 14)
        }
    }

}
