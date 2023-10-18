use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{RelativePosition, generate_valid_moves::GenerateValidMoves}};

use super::Piece;

pub struct King{
    color: Color,
}

impl Piece for King{
    fn new(color: Color) -> Self {
        King{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        0
    }

    fn prefix(&self) -> String {
        String::from("K")
    }

    fn icon(&self) -> char{
        '♔'
    }

    fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        (Vec::new(), position.file.abs() <= 1 && position.rank.abs() <= 1)
    }
}

impl GenerateValidMoves for King{
    fn generate_valid_moves(&self) -> Vec<RelativePosition>{
        return vec![RelativePosition {file : 1, rank : 1}];
    }
}


impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_should_only_move_to_adjacent_tiles(){
        let king = King::new(Color::White);

        let mut adjacent_positions = Vec::new();

        for file in -1..=1{
            for rank in -1..=1{
                adjacent_positions.push(RelativePosition{file, rank});
            }
        }

        for position in adjacent_positions{
            let (movement_path, valid_movement) = king.valid_move(&position);
            assert_eq!(valid_movement, true);
            assert_eq!(movement_path.len(), 0);
        }
    }

    #[test]
    fn it_should_not_move_to_distant_tiles(){
        let king = King::new(Color::White);

        let mut distant_positions = Vec::new();

        for file in -2i8..=2{
            for rank in -2i8..=2{
                if file.abs() > 1 || rank.abs() > 1{
                    distant_positions.push(RelativePosition{file, rank});
                }
            }
        }

        for position in distant_positions{
            let (movement_path, valid_movement) = king.valid_move(&position);
            assert_eq!(valid_movement, false);
            assert_eq!(movement_path.len(), 0);
        }

    }
}