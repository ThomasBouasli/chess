use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::RelativePosition};

use super::Piece;

pub struct Pawn{
    color: Color,
}

impl Piece for Pawn{
    fn new(color: Color) -> Self {
        Pawn{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        1
    }

    fn prefix(&self) -> String {
        String::from("P")
    }

    fn icon(&self) -> char{
        '♙'
    }


    fn valid_move(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file != 0{
            (Vec::new(), false)
        }else if position.rank == 1 * self.multiplier(){
            (Vec::new(), true)
        }else if position.rank == 2 * self.multiplier(){
            (vec![RelativePosition {file: 0, rank: 1 * self.multiplier()}], true)
        }else{
            (Vec::new(), false)
        }
    }

    fn valid_capture(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file.abs() == 1 && position.rank == 1{
            (Vec::new(), true)
        }else{
            (Vec::new(), false) 
        }
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}

mod tests{
    use super::*;

    #[test]
    fn test_pawn_can_move_one_square_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 1});
        assert_eq!(valid, true, "Pawn should be able to move one square forward");
        assert_eq!(move_path.len(), 0, "Valid moves should not contain a paths");
    }

    #[test]
    fn test_pawn_can_move_two_squares_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 2});
        assert_eq!(valid, true, "Pawn should be able to move two squares forward");
        assert_eq!(move_path.len(), 1, "Valid moves should contain a paths");
    }

    #[test]
    fn test_pawn_cannot_move_three_squares_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 3});
        assert_eq!(valid, false, "Pawn should not be able to move three squares forward");
        assert_eq!(move_path.len(), 0, "Invalid moves should not contain a paths");
    }

    #[test]
    fn test_pawn_cannot_move_backwards(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: -1});
        assert_eq!(valid, false, "Pawn should not be able to move backwards");
        assert_eq!(move_path.len(), 0, "Invalid moves should not contain a paths");
    }

    #[test]
    fn test_pawn_can_capture_diagonally(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_capture(&RelativePosition{file: 1, rank: 1});
        assert_eq!(valid, true, "Pawn should be able to capture diagonally");
        assert_eq!(move_path.len(), 0, "Moves that are one square of distance should not contain a path");
    }

    #[test]
    fn test_pawn_cannot_capture_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_capture(&RelativePosition{file: 0, rank: 1});
        assert_eq!(valid, false, "Pawn should not be able to capture forward");
        assert_eq!(move_path.len(), 0, "Invalid moves should not contain a paths");
    }
}