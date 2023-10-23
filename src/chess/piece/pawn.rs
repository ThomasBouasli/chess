use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition}};

use super::{Piece, PieceType};


#[derive(Clone)]

pub struct Pawn{
    color: Color,
    is_first_move : bool,
}

impl Pawn{
    pub fn new(color: Color) -> Self {
        Pawn{color, is_first_move : true}
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn value(&self) -> u8 {
        1
    }

    pub fn prefix(&self) -> char {
        'P'
    }

    pub fn icon(&self) -> char{
        '♙'
    }

    pub fn moved(&mut self){
        self.is_first_move = false;
    }

    pub fn has_moved(&self) -> bool{
        !self.is_first_move
    }

    pub fn multiplier(&self) -> i8{
        match self.color{
            Color::White => 1,
            Color::Black => -1,
        }
    }

    pub fn valid_move(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file != 0{
            (Vec::new(), false)
        }else if position.rank == 1 * self.multiplier(){
            (Vec::new(), true)
        }else if position.rank == 2 * self.multiplier() && self.is_first_move{
            (vec![RelativePosition {file: 0, rank: 1 * self.multiplier()}], true)
        }else{
            (Vec::new(), false)
        }
    }

    pub fn valid_capture(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file.abs() == 1 && position.rank == 1{
            (Vec::new(), true)
        }else{
            (Vec::new(), false) 
        }
    }

    pub fn generate_valid_captures(&self) -> Vec<RelativePosition>{
        let mut moves = Vec::new();

        moves.push(RelativePosition{file: 1, rank: 1 * self.multiplier()});
        moves.push(RelativePosition{file: -1, rank: 1 * self.multiplier()});

        moves
    }

    pub fn generate_valid_moves(&self) -> Vec<RelativePosition>{
        let mut moves = Vec::new();

        moves.push(RelativePosition{file: 0, rank: 1 * self.multiplier()});

        if self.is_first_move{
            moves.push(RelativePosition{file: 0, rank: 2 * self.multiplier()});
        }

        moves
    }

    pub fn promote(&self, piece_type : PieceType) -> Piece{
        match piece_type{
            PieceType::Queen => Piece::Queen{piece: super::queen::Queen::new(self.color)},
            PieceType::Rook => Piece::Rook{piece: super::rook::Rook::new(self.color)},
            PieceType::Bishop => Piece::Bishop{piece: super::bishop::Bishop::new(self.color)},
            PieceType::Knight => Piece::Knight{piece: super::knight::Knight::new(self.color)},
            _ => panic!("Invalid piece type"),
        }
    }
}

impl GenerateValidMoves for Pawn{
    fn generate_valid_plays(&self) -> Vec<RelativePosition>{
        let mut moves = Vec::new();

        moves.append(&mut self.generate_valid_captures());
        moves.append(&mut self.generate_valid_moves());

        moves
    }
}


impl Display for Pawn {
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
    fn test_pawn_can_move_one_tile_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 1});
        assert_eq!(valid, true, "Pawn should be able to move one tile forward");
        assert_eq!(move_path.len(), 0, "Valid moves should not contain a paths");
    }

    #[test]
    fn test_pawn_can_move_two_tiles_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 2});
        assert_eq!(valid, true, "Pawn should be able to move two tiles forward");
        assert_eq!(move_path.len(), 1, "Valid moves should contain a paths");
    }
    
    #[test]
    fn test_pawn_cannot_move_two_tiles_forward_if_not_first_move(){
        let mut pawn = Pawn::new(Color::White);

        pawn.moved();

        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 2});

        assert_eq!(valid, false, "Pawn should not be able to move two tiles forward if it's not it's first move");
        assert_eq!(move_path.len(), 0, "Invalid moves should not contain a paths");
    }

    #[test]
    fn test_pawn_cannot_move_three_tiles_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_move(&RelativePosition{file: 0, rank: 3});
        assert_eq!(valid, false, "Pawn should not be able to move three tiles forward");
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
        assert_eq!(move_path.len(), 0, "Moves that are one tile of distance should not contain a path");
    }

    #[test]
    fn test_pawn_cannot_capture_forward(){
        let pawn = Pawn::new(Color::White);
        let (move_path, valid) = pawn.valid_capture(&RelativePosition{file: 0, rank: 1});
        assert_eq!(valid, false, "Pawn should not be able to capture forward");
        assert_eq!(move_path.len(), 0, "Invalid moves should not contain a paths");
    }


    #[test]
    fn test_generated_moves_should_be_valid(){
        let pawn = Pawn::new(Color::White);

        let generated_moves = pawn.generate_valid_plays();

        for movement in generated_moves{
            assert!(pawn.valid_move(&movement).1 || pawn.valid_capture(&movement).1);
        }
    }

    #[test]
    fn test_if_there_are_not_any_missing_valid_moves(){
        let pawn = Pawn::new(Color::White);

        let generated_moves = pawn.generate_valid_plays();

        let mut possible_moves = Vec::new();

        for file in -7i8..=7{
            for rank in -7i8..=7{
                let (_, valid) = pawn.valid_move(&RelativePosition{file, rank});
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