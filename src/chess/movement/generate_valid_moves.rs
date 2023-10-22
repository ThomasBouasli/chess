use super::relative_position::RelativePosition;

pub trait GenerateValidMoves{
    fn generate_valid_moves(&self) -> Vec<RelativePosition>;
}