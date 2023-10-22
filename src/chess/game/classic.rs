use crate::chess::board::Board;
use crate::chess::movement::chess_notation::ChessNotationPosition;
use crate::chess::piece::PieceType;
use crate::chess::piece::Piece;
use crate::chess::color::Color;

use super::Game;

pub struct ClassicGame{}

impl ClassicGame{
    pub fn new() -> Game{

        let mut board = Board::new();

        for n in 0..8{
            board.get_tile_mut(&ChessNotationPosition::new((n as u8 + 97) as char, 2).to_position()).set_piece(Piece::new(Color::White, PieceType::Pawn));
            board.get_tile_mut(&ChessNotationPosition::new((n as u8 + 97) as char, 7).to_position()).set_piece(Piece::new(Color::Black, PieceType::Pawn));
        }
    
        board.get_tile_mut(&ChessNotationPosition::new('a', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Rook));
        board.get_tile_mut(&ChessNotationPosition::new('b', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Knight));
        board.get_tile_mut(&ChessNotationPosition::new('c', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Bishop));
        board.get_tile_mut(&ChessNotationPosition::new('d', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Queen));
        board.get_tile_mut(&ChessNotationPosition::new('e', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::King));
        board.get_tile_mut(&ChessNotationPosition::new('f', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Bishop));
        board.get_tile_mut(&ChessNotationPosition::new('g', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Knight));
        board.get_tile_mut(&ChessNotationPosition::new('h', 1).to_position()).set_piece(Piece::new(Color::White, PieceType::Rook));


        board.get_tile_mut(&ChessNotationPosition::new('a', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Rook));
        board.get_tile_mut(&ChessNotationPosition::new('b', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Knight));
        board.get_tile_mut(&ChessNotationPosition::new('c', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Bishop));
        board.get_tile_mut(&ChessNotationPosition::new('d', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Queen));
        board.get_tile_mut(&ChessNotationPosition::new('e', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::King));
        board.get_tile_mut(&ChessNotationPosition::new('f', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Bishop));
        board.get_tile_mut(&ChessNotationPosition::new('g', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Knight));
        board.get_tile_mut(&ChessNotationPosition::new('h', 8).to_position()).set_piece(Piece::new(Color::Black, PieceType::Rook));


        Game::new(
            board,
            Color::White,
        )
    }
}