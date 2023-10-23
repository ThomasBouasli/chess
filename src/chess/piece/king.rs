use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{ generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition}};

#[derive(Clone)]

pub struct King{
    color: Color,
    is_first_move: bool,
}

impl King{
    pub fn new(color: Color) -> Self {
        King{color , is_first_move: true}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> u8 {
        0
    }

    pub fn prefix(&self) -> char {
        'K'
    }

    pub fn icon(&self) -> char{
        '♔'
    }

    pub fn moved(&mut self){
        self.is_first_move = false;
    }

    pub fn has_moved(&self) -> bool{
        !self.is_first_move
    }

    pub fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        (Vec::new(), position.file.abs() <= 1 && position.rank.abs() <= 1 && (position.file != 0 || position.rank != 0))
    }

    pub fn valid_capture(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.valid_move(position)
    }

    pub fn castle_queen_side(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool){
        if position.rank == 0 && position.file == -2{
            let vec = vec![RelativePosition{file: -1, rank: 0}, RelativePosition{file: -2, rank: 0}];

            return (vec, true);
        }

        return (Vec::new(), false);
    }

    pub fn castle_king_side(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool){
        if position.rank == 0 && position.file == 2{
            let vec = vec![RelativePosition{file: 1, rank: 0}, RelativePosition{file: 2, rank: 0}];

            return (vec, true);
        }

        return (Vec::new(), false);
    }
}

impl GenerateValidMoves for King{
    fn generate_valid_plays(&self) -> Vec<RelativePosition>{
        let mut moves = Vec::new();

        for file in -1i8..=1{
            for rank in -1i8..=1{
                if file != 0 || rank != 0{
                    moves.push(RelativePosition{file, rank});
                }
            }
        }

        moves
    }
}


impl Display for King {
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
    fn it_should_only_move_to_adjacent_tiles(){
        let king = King::new(Color::White);

        let mut adjacent_positions = Vec::new();

        for file in -1..=1{
            for rank in -1..=1{
                if file != 0 || rank != 0{
                    adjacent_positions.push(RelativePosition{file, rank});
                }
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


    #[test]
    fn test_generated_moves_should_be_valid(){
        let king = King::new(Color::White);

        let generated_moves = king.generate_valid_plays();

        for movement in generated_moves{
            assert!(king.valid_move(&movement).1 || king.valid_capture(&movement).1);
        }
    }

    #[test]
    fn test_if_there_are_not_any_missing_valid_moves(){
        let king = King::new(Color::White);

        let generated_moves = king.generate_valid_plays();

        let mut possible_moves = Vec::new();

        for file in -7i8..=7{
            for rank in -7i8..=7{
                let (_, valid) = king.valid_move(&RelativePosition{file, rank});
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