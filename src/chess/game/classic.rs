use crate::chess::board::Board;
use crate::chess::movement::ChessNotationPosition;
use crate::chess::piece::{Piece,bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};
use crate::chess::color::Color;

use super::Game;

pub struct ClassicGame{}

impl ClassicGame{
    pub fn new() -> Game{

        let mut board = Board::new();

        for n in 0..8{
            board.get_tile_mut(&ChessNotationPosition::new((n as u8 + 97) as char, 2).to_position()).set_piece(Box::new(Pawn::new(Color::White)));
            board.get_tile_mut(&ChessNotationPosition::new((n as u8 + 97) as char, 7).to_position()).set_piece(Box::new(Pawn::new(Color::Black)));
        }
    
        board.get_tile_mut(&ChessNotationPosition::new('a', 1).to_position()).set_piece(Box::new(Rook::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('b', 1).to_position()).set_piece(Box::new(Knight::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('c', 1).to_position()).set_piece(Box::new(Bishop::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('d', 1).to_position()).set_piece(Box::new(Queen::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('e', 1).to_position()).set_piece(Box::new(King::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('f', 1).to_position()).set_piece(Box::new(Bishop::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('g', 1).to_position()).set_piece(Box::new(Knight::new(Color::White)));
        board.get_tile_mut(&ChessNotationPosition::new('h', 1).to_position()).set_piece(Box::new(Rook::new(Color::White)));


        board.get_tile_mut(&ChessNotationPosition::new('a', 8).to_position()).set_piece(Box::new(Rook::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('b', 8).to_position()).set_piece(Box::new(Knight::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('c', 8).to_position()).set_piece(Box::new(Bishop::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('d', 8).to_position()).set_piece(Box::new(Queen::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('e', 8).to_position()).set_piece(Box::new(King::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('f', 8).to_position()).set_piece(Box::new(Bishop::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('g', 8).to_position()).set_piece(Box::new(Knight::new(Color::Black)));
        board.get_tile_mut(&ChessNotationPosition::new('h', 8).to_position()).set_piece(Box::new(Rook::new(Color::Black)));


        Game{
            board,
            turn: Color::White,
            moves: Vec::new(),
        }
    }
}