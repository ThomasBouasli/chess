use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{ line::LineMovement,generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition}};


#[derive(Clone)]

pub struct Rook{
    color: Color,
}

impl LineMovement for Rook{}

impl Rook{
    pub fn new(color: Color) -> Self {
        Rook{color}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> u8 {
        5
    }

    pub fn prefix(&self) -> char {
        'R'
    }

    pub fn icon(&self) -> char{
        '♖'
    }

    pub fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.line_movement(position)
    }

    pub fn valid_capture(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.valid_move(position)
    }
}

impl GenerateValidMoves for Rook{
    fn generate_valid_moves(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();

        for dx in -7i8..=7 {
            for dy in -7i8..=7 {
                if (dx != 0 && dy == 0) || (dx == 0 && dy != 0) {
                    moves.push(RelativePosition { file: dx, rank: dy });
                }
            }
        }

        moves
    }
}


impl Display for Rook {
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
        let rook = Rook::new(Color::White);

        let generated_moves = rook.generate_valid_moves();

        for movement in generated_moves{
            assert!(rook.valid_move(&movement).1 || rook.valid_capture(&movement).1);
        }
    }

    #[test]
    fn test_if_there_are_not_any_missing_valid_moves(){
        let rook = Rook::new(Color::White);

        let generated_moves = rook.generate_valid_moves();

        let mut possible_moves = Vec::new();

        for file in -7i8..=7{
            for rank in -7i8..=7{
                let (_, valid) = rook.valid_move(&RelativePosition{file, rank});
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