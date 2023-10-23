use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{ generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition}};


#[derive(Clone)]

pub struct Knight{
    color: Color,
}

impl Knight{
    pub fn new(color: Color) -> Self {
        Knight{color}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> u8 {
        3
    }

    pub fn prefix(&self) -> char {
        'N'
    }

    pub fn icon(&self) -> char{
        '♘'
    }

    pub fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if (position.file.abs() == 2 && position.rank.abs() == 1) || (position.file.abs() == 1 && position.rank.abs() == 2){
            (Vec::new(), true)
        }else{
            (Vec::new(), false)
        }
    }

    pub fn valid_capture(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.valid_move(position)
    }
}

impl GenerateValidMoves for Knight{
    fn generate_valid_plays(&self) -> Vec<RelativePosition>{
        let mut moves = Vec::new();

        for file in -2i8..=2{
            for rank in -2i8..=2{
                if file.abs() != rank.abs() && file != 0 && rank != 0{
                    moves.push(RelativePosition{file, rank});
                }
            }
        }

        moves
    }
}


impl Display for Knight {
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
    fn test_knight_valid_move(){
        let knight = Knight::new(Color::White);

        let mut possible_moves = Vec::new();

        for file in -2i8..=2{
            for rank in -2i8..=2{
                if file.abs() != rank.abs() && file != 0 && rank != 0{
                    possible_moves.push(RelativePosition{file, rank});
                }
            }
        }

        for position in possible_moves{
            let (moves, valid) = knight.valid_move(&position);
            assert!(valid, "Knight should be able to move to {:?}", position);
            assert_eq!(moves.len(), 0, "Knight should not be able to move to {:?}", position);
        }
    }


    #[test]
    fn test_generated_moves_should_be_valid(){
        let knight = Knight::new(Color::White);

        let generated_moves = knight.generate_valid_plays();

        for movement in generated_moves{
            assert!(knight.valid_move(&movement).1 || knight.valid_capture(&movement).1);
        }
    }

    #[test]
    fn test_if_there_are_not_any_missing_valid_moves(){
        let knight = Knight::new(Color::White);

        let generated_moves = knight.generate_valid_plays();

        let mut possible_moves = Vec::new();

        for file in -7i8..=7{
            for rank in -7i8..=7{
                let (_, valid) = knight.valid_move(&RelativePosition{file, rank});
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