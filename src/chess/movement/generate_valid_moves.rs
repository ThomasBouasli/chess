use super::relative_position::RelativePosition;

pub trait GenerateValidMoves{
    fn generate_valid_plays(&self) -> Vec<RelativePosition>;
}