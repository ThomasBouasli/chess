use std::fmt::Display;

pub struct RelativePosition{
    pub file: i8,
    pub rank: i8,
}

impl RelativePosition{
    pub fn to_absolute(&self, position: &Position) -> Position {
        Position {
            file: (position.file as i8 + self.file) as usize,
            rank: (position.rank as i8 + self.rank) as usize,
        }
    }
}

impl Display for RelativePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

pub struct  Position{
    pub file: usize,
    pub rank: usize,
}


impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.file as u8 + 97) as char, self.rank + 1)
    }
}

impl Position {
    pub fn from_chess_notation(pos: ChessNotationPosition) -> Self {
        Self {
            file: pos.file as usize - 97,
            rank: pos.rank as usize - 1,
        }
    }

    pub fn to_chess_notation(&self) -> ChessNotationPosition {
        ChessNotationPosition {
            file: (self.file as u8 + 97) as char,
            rank: self.rank as u8 + 1,
        }
    }
}

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

    pub fn from_position(pos: Position) -> Self {
        Self {
            file: (pos.file as u8 + 97) as char,
            rank: pos.rank as u8 + 1,
        }
    }

    pub fn to_position(&self) -> Position {
        Position {
            file: self.file as usize - 97,
            rank: self.rank as usize - 1,
        }
    }
}


pub struct Movement{
    from: Position,
    to: Position,
} 


impl Movement {
    pub fn new(from: Position, to: Position) -> Self {
        Self {
            from,
            to,
        }
    }

    pub fn from(&self) -> &Position {
        &self.from
    }

    pub fn to(&self) -> &Position {
        &self.to
    }
}