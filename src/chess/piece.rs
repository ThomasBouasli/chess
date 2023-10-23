use std::fmt::Display;

use self::{knight::Knight, bishop::Bishop, rook::Rook, pawn::Pawn, queen::Queen, king::King};

use super::movement::{generate_valid_moves::GenerateValidMoves, relative_position::RelativePosition};


pub mod pawn;
pub mod king;
pub mod bishop;
pub mod knight;
pub mod rook;
pub mod queen;

#[derive(Clone)]
pub enum Piece{
    King {piece: King},
    Queen {piece: Queen},
    Knight {piece: Knight},
    Bishop {piece: Bishop},
    Rook {piece: Rook},
    Pawn {piece: Pawn},
}


pub enum PieceType{
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
}

impl Piece {
    pub fn new(color: super::color::Color, piece: PieceType) -> Self {
        match piece{
            PieceType::King => Piece::King{piece: King::new(color)},
            PieceType::Queen => Piece::Queen{piece: Queen::new(color)},
            PieceType::Knight => Piece::Knight{piece: Knight::new(color)},
            PieceType::Bishop => Piece::Bishop{piece: Bishop::new(color)},
            PieceType::Rook => Piece::Rook{piece: Rook::new(color)},
            PieceType::Pawn => Piece::Pawn{piece: Pawn::new(color)},
        }
    }
    
    pub fn color(&self) -> &super::color::Color {
        match self{
            Piece::King{piece} => piece.color(),
            Piece::Queen{piece} => piece.color(),
            Piece::Knight{piece} => piece.color(),
            Piece::Bishop{piece} => piece.color(),
            Piece::Rook{piece} => piece.color(),
            Piece::Pawn{piece} => piece.color(),
        }
    }

    pub fn prefix(&self) -> char {
        match self{
            Piece::King{piece} => piece.prefix(),
            Piece::Queen{piece} => piece.prefix(),
            Piece::Knight{piece} => piece.prefix(),
            Piece::Bishop{piece} => piece.prefix(),
            Piece::Rook{piece} => piece.prefix(),
            Piece::Pawn{piece} => piece.prefix(),
        }
    }

    pub fn value(&self) -> u8 {
        match self{
            Piece::King{piece} => piece.value(),
            Piece::Queen{piece} => piece.value(),
            Piece::Knight{piece} => piece.value(),
            Piece::Bishop{piece} => piece.value(),
            Piece::Rook{piece} => piece.value(),
            Piece::Pawn{piece} => piece.value(),
        }
    }

    pub fn castle_queen_side(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        match self{
            Piece::King{piece} => piece.castle_queen_side(position),
            _ => (Vec::new(), false),
        }
    }

    pub fn castle_king_side(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        match self{
            Piece::King{piece} => piece.castle_king_side(position),
            _ => (Vec::new(), false),
        }
    }

    pub fn valid_move(&self, position: &super::movement::relative_position::RelativePosition) -> (Vec<super::movement::relative_position::RelativePosition>, bool) {
        match self{
            Piece::King{piece} => piece.valid_move(position),
            Piece::Queen{piece} => piece.valid_move(position),
            Piece::Knight{piece} => piece.valid_move(position),
            Piece::Bishop{piece} => piece.valid_move(position),
            Piece::Rook{piece} => piece.valid_move(position),
            Piece::Pawn{piece} => piece.valid_move(position),
        }
    }

    pub fn valid_capture(&self,  position: &super::movement::relative_position::RelativePosition) -> (Vec<super::movement::relative_position::RelativePosition>, bool) {
        match self{
            Piece::King{piece} => piece.valid_capture(position),
            Piece::Queen{piece} => piece.valid_capture(position),
            Piece::Knight{piece} => piece.valid_capture(position),
            Piece::Bishop{piece} => piece.valid_capture(position),
            Piece::Rook{piece} => piece.valid_capture(position),
            Piece::Pawn{piece} => piece.valid_capture(position),
        }
    }

    pub fn moved(&mut self){
        if let Piece::Pawn{piece} = self{
            piece.moved();
        }
    }

    pub fn has_moved(&self) -> bool{
        match self{
            Piece::King{piece} => piece.has_moved(),
            Piece::Rook{piece} => piece.has_moved(),
            Piece::Pawn{piece} => piece.has_moved(),
            _ => false,
        }
    }

    pub fn generate_valid_moves(&self) -> Vec<super::movement::relative_position::RelativePosition> {
        match self{
            Piece::King{piece} => piece.generate_valid_plays(),
            Piece::Queen{piece} => piece.generate_valid_plays(),
            Piece::Knight{piece} => piece.generate_valid_plays(),
            Piece::Bishop{piece} => piece.generate_valid_plays(),
            Piece::Rook{piece} => piece.generate_valid_plays(),
            Piece::Pawn{piece} => piece.generate_valid_moves(),
        }
    }
    
    pub fn generate_valid_captures(&self) -> Vec<super::movement::relative_position::RelativePosition> {
        match self{
            Piece::King{piece} => piece.generate_valid_plays(),
            Piece::Queen{piece} => piece.generate_valid_plays(),
            Piece::Knight{piece} => piece.generate_valid_plays(),
            Piece::Bishop{piece} => piece.generate_valid_plays(),
            Piece::Rook{piece} => piece.generate_valid_plays(),
            Piece::Pawn{piece} => piece.generate_valid_captures(),
        }
    }

    pub fn promote(&self, piece_type : PieceType) -> Piece{
        match self{
            Piece::Pawn{piece} => piece.promote(piece_type),
            _ => panic!("Invalid piece type"),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Piece::King{piece} => write!(f, "{}", piece),
            Piece::Queen{piece} => write!(f, "{}", piece),
            Piece::Knight{piece} => write!(f, "{}", piece),
            Piece::Bishop{piece} => write!(f, "{}", piece),
            Piece::Rook{piece} => write!(f, "{}", piece),
            Piece::Pawn{piece} => write!(f, "{}", piece),
        }
    }
}

impl GenerateValidMoves for Piece{
    fn generate_valid_plays(&self) -> Vec<super::movement::relative_position::RelativePosition> {
        match self{
            Piece::King{piece} => piece.generate_valid_plays(),
            Piece::Queen{piece} => piece.generate_valid_plays(),
            Piece::Knight{piece} => piece.generate_valid_plays(),
            Piece::Bishop{piece} => piece.generate_valid_plays(),
            Piece::Rook{piece} => piece.generate_valid_plays(),
            Piece::Pawn{piece} => piece.generate_valid_plays(),
        }
    }
}

