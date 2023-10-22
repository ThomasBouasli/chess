use self::absolute_position::AbsolutePosition;

pub mod line;
pub mod diagonal;
pub mod generate_valid_moves;
pub mod absolute_position;
pub mod relative_position;
pub mod chess_notation;



#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Movement{
    from: AbsolutePosition,
    to: AbsolutePosition,
} 


impl Movement {
    pub fn new(from: AbsolutePosition, to: AbsolutePosition) -> Self {
        Self {
            from,
            to,
        }
    }

    pub fn from(&self) -> &AbsolutePosition {
        &self.from
    }

    pub fn to(&self) -> &AbsolutePosition {
        &self.to
    }
}

