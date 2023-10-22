use super::absolute_position::AbsolutePosition;


#[derive(Debug)]
pub struct ChessNotationPosition{
    pub file: char,
    pub rank: u8,
}

impl ChessNotationPosition {
    pub fn new(file: char, rank: u8) -> Self {
        Self {
            file,
            rank,
        }
    }

    pub fn to_position(&self) -> AbsolutePosition {
        AbsolutePosition {
            file: self.file as usize - 97,
            rank: self.rank as usize - 1,
        }
    }

    pub fn from_position(position: AbsolutePosition) -> Self {
        Self {
            file: (position.file as u8 + 97) as char,
            rank: (position.rank as u8 + 1) as u8,
        }
    }

    pub fn from_str(string: &str) -> Result<Self, String> {
        if string.len() != 2 {
            return Err(String::from("Invalid string length"));
        }

        let file = string.chars().nth(0).unwrap();
        let rank = string.chars().nth(1).unwrap();

        if !file.is_ascii_alphabetic() {
            return Err(String::from("Invalid file"));
        }

        if !rank.is_ascii_digit() {
            return Err(String::from("Invalid rank"));
        }

        Ok(Self::new(file, rank.to_digit(10).unwrap() as u8))
    }
}
