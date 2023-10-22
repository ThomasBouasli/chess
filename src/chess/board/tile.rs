use std::fmt::Display;

use crate::chess::piece::Piece;

#[derive(Clone)]
pub struct Tile{
    piece: Option<Piece>,
}

impl Tile{
    pub fn new() -> Self {
        Tile{piece: None}
    }

    pub fn set_piece(&mut self, piece: Piece){
        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> &Option<Piece>{
        &self.piece
    }

    pub fn remove_piece(&mut self) -> Option<Piece>{
        self.piece.take()
    }
}

impl Display for Tile{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.piece{
            Some(piece) => write!(f, "{}", piece),
            None => write!(f, "   "),
        }
    }
}