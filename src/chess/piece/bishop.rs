use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{ diagonal::DiagonalMovement, generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition}};

#[derive(Clone)]
pub struct Bishop{
    color: Color,
}

impl DiagonalMovement for Bishop{}

impl Bishop{
    pub fn new(color: Color) -> Self {
        Bishop{color}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> u8 {
        3
    }

    pub fn prefix(&self) -> char {
        'B'
    }

    pub fn icon(&self) -> char{
        '♗'
    }

    pub fn valid_move(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.diagonal_movement(position)
    }

    pub fn valid_capture(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.valid_move(position)
    }
}

impl GenerateValidMoves for Bishop{
    fn generate_valid_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();

        for dx in -7i8..=7 {
            for dy in -7i8..=7 {
                if dx != 0 && dy != 0 && dx.abs() == dy.abs() {
                    moves.push(RelativePosition { file: dx, rank: dy });
                }
            }
        }

        moves
    }
}


impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.color() {
            Color::White => {
                format!(" {} ", self.icon())
                .fmt(f)
            },
            Color::Black => {
                format!(" {} ", self.icon())
                .yellow()
                .fmt(f)
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_generated_moves_should_be_valid(){
        let bishop = Bishop::new(Color::White);

        let generated_moves = bishop.generate_valid_moves();

        for movement in generated_moves{
            assert!(bishop.valid_move(&movement).1 || bishop.valid_capture(&movement).1);
        }
    }

    #[test]
    fn test_if_there_are_not_any_missing_valid_moves(){
        let bishop = Bishop::new(Color::White);

        let generated_moves = bishop.generate_valid_moves();

        let mut possible_moves = Vec::new();

        for file in -7i8..=7{
            for rank in -7i8..=7{
                let (_, valid) = bishop.valid_move(&RelativePosition{file, rank});
                if valid{
                    possible_moves.push(RelativePosition{file, rank});
                }
            }
        }

        for position in possible_moves{
            assert!(generated_moves.contains(&position), "Missing move: {:?}", position);
        }
    }
}