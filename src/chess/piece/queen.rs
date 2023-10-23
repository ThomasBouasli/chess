use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{ diagonal::DiagonalMovement, line::LineMovement, generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition}};


#[derive(Clone)]

pub struct Queen{
    color: Color,
}

impl DiagonalMovement for Queen{}

impl LineMovement for Queen{}

impl Queen{
    pub fn new(color: Color) -> Self {
        Queen{color}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> u8 {
        9
    }

    pub fn prefix(&self) -> char {
        'Q'
    }

    pub fn icon(&self) -> char{
        '♕'
    }

    pub fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        let ( diagonal_path, diagonal_can_move) = self.diagonal_movement(position);
        let ( line_path, line_can_move) = self.line_movement(position);

        let mut movement_path = Vec::new();

        movement_path.extend(diagonal_path);
        movement_path.extend(line_path);

        (movement_path, diagonal_can_move || line_can_move)
    }

    pub fn valid_capture(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.valid_move(position)
    }
}

impl GenerateValidMoves for Queen{
    fn generate_valid_plays(&self) -> Vec<RelativePosition> {
        let mut moves = Vec::new();

        for dx in -7i8..=7 {
            for dy in -7i8..=7 {
                if dx != 0 && dy != 0 && dx.abs() == dy.abs() {
                    moves.push(RelativePosition { file: dx, rank: dy });
                }
                if (dx != 0 && dy == 0) || (dx == 0 && dy != 0) {
                    moves.push(RelativePosition { file: dx, rank: dy });
                }
            }
        }

        moves
    }
}

impl Display for Queen {
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
    fn test_should_generate_valid_moves() {
        let queen = Queen::new(Color::White);

        // Get the valid moves for the Queen
        let valid_moves = queen.generate_valid_plays();

        let expected_moves: Vec<RelativePosition> = vec![
            RelativePosition { file: 1, rank: 1 },
            RelativePosition { file: 2, rank: 2 },
            RelativePosition { file: -1, rank: 1 },
        ];

        for expected_move in &expected_moves {
            assert!(valid_moves.contains(expected_move), "Missing move: {:?}", expected_move);
        }

        assert_eq!(valid_moves.len(), 56);

        let unexpected_moves: Vec<RelativePosition> = vec![
            RelativePosition { file: 2, rank: 1 },
            RelativePosition { file: -3, rank: 2 },
        ];

        for unexpected_move in &unexpected_moves {
            assert!(!valid_moves.contains(unexpected_move), "Unexpected move found: {:?}", unexpected_move);
        }
    }

    #[test]
    fn test_generated_moves_should_be_valid(){
        let queen = Queen::new(Color::White);

        let generated_moves = queen.generate_valid_plays();

        for movement in generated_moves{
            assert!(queen.valid_move(&movement).1 || queen.valid_capture(&movement).1);
        }
    }

    #[test]
    fn test_if_there_are_not_any_missing_valid_moves(){
        let queen = Queen::new(Color::White);

        let generated_moves = queen.generate_valid_plays();

        let mut possible_moves = Vec::new();

        for file in -7i8..=7{
            for rank in -7i8..=7{
                let (_, valid) = queen.valid_move(&RelativePosition{file, rank});
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