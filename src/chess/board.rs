use std::fmt::Display;

use colored::Colorize;

use self::tile::Tile;

use super::movement::absolute_position::AbsolutePosition;

pub mod tile;

#[derive(Clone)]
pub struct Board{
    tiles: Vec<Vec<tile::Tile>>,
}

impl Board{ 
    pub fn new() -> Self {
        let mut tile = Vec::new();
        for _ in 0..8{
            let mut rank = Vec::new();
            for _ in 0..8{
                rank.push(tile::Tile::new());
            }
            tile.push(rank);
        }
        Board{tiles: tile}
    }

    pub fn get_tile_mut(&mut self, position : &AbsolutePosition) -> &mut Tile{
        &mut self.tiles[position.file][position.rank]
    }

    pub fn get_tile(&self, position : &AbsolutePosition) -> &Tile{
        &self.tiles[position.file][position.rank]
    }

    pub fn get_king_position(&self, color: &super::color::Color) -> Option<AbsolutePosition>{
        for (tile, position) in self.get_tiles(){
            if let Some(piece) = tile.get_piece(){
                if piece.color() == color && piece.prefix() == 'K'{
                    return Some(position);
                }
            }
        }
        None
    }

    pub fn get_tiles(&self) -> Vec<(&Tile, AbsolutePosition)>{
        self.tiles.iter().enumerate().flat_map(|(file_i, rank)|{
            rank.iter().enumerate().map(move |(rank_i, tile)|{
                (tile, AbsolutePosition{file: file_i, rank: rank_i})
            })
        }).collect()
    }
}

impl Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for (rank_i, _) in self.tiles.iter().enumerate().rev(){
            board.push_str(&format!("{} ",rank_i + 1));
            for (file_i, _) in self.tiles[rank_i].iter().enumerate(){
                if (rank_i + file_i) % 2 == 0{
                    board.push_str(&format!("{}", self.tiles[file_i][rank_i]).on_bright_black().to_string());
                }else{
                    board.push_str(&format!("{}", self.tiles[file_i][rank_i]).on_bright_white().to_string());
                }
            }
            board.push('\n');
        }

        board.push_str("   a  b  c  d  e  f  g  h\n");

        write!(f, "{}", board)
    }
}

#[cfg(test)]
mod tests{
    use crate::chess::color::Color;
    use crate::chess::movement::chess_notation::ChessNotationPosition;
    use crate::chess::piece::{Piece, PieceType};
    use super::*;

    #[test]
    fn test_board_get_tile(){
        let board = Board::new();
        let tile = board.get_tile(&AbsolutePosition::new(0, 0));
        
        let piece = tile.get_piece();

        assert!(piece.is_none());
    }

    #[test]
    fn test_board_get_tile_mut(){
        let mut board = Board::new();
        let tile = board.get_tile_mut(&AbsolutePosition::new(0, 0));
        
        tile.set_piece(Piece::new(Color::White, PieceType::Pawn));

        let piece = tile.get_piece();

        assert!(piece.is_some());
    }

    #[test]
    fn test_should_get_king_position(){
        let king = Piece::new(Color::White, PieceType::King);
        let king_position = ChessNotationPosition::new('e', 1).to_position();

        let mut board = Board::new();

        board.get_tile_mut(&king_position).set_piece(king);

        let got_king_position = match board.get_king_position(&Color::White){
            Some(position) => position,
            None => panic!("King not found!"),
        };

        assert_eq!(got_king_position, king_position);
    }

    #[test]
    fn test_should_not_find_missing_king(){
        let board = Board::new();

        let got_king_position = board.get_king_position(&Color::White);

        assert!(got_king_position.is_none());
    }
}