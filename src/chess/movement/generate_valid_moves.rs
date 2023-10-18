use super::RelativePosition;

pub trait GenerateValidMoves{
    fn generate_valid_moves(&self) -> Vec<RelativePosition>;
}