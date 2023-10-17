use std::fmt::Display;

use self::square::Square;

use super::movement::Position;

pub mod square;

pub struct Board{
    tiles: Vec<Vec<square::Square>>,
}

impl Board{ 
    pub fn new() -> Self {
        let mut square = Vec::new();
        for _ in 0..8{
            let mut rank = Vec::new();
            for _ in 0..8{
                rank.push(square::Square::new());
            }
            square.push(rank);
        }
        Board{tiles: square}
    }

    pub fn get_tile_mut(&mut self, position : &Position) -> &mut Square{
        &mut self.tiles[position.file][position.rank]
    }

    pub fn get_tile(&self, position : &Position) -> &Square{
        &self.tiles[position.file][position.rank]
    }
}

impl Display for Board{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for (rank_i, _) in self.tiles.iter().enumerate().rev(){
            board.push_str(&format!("{} ",rank_i + 1));
            for (file_i, _) in self.tiles[rank_i].iter().enumerate(){
                if (rank_i + file_i) % 2 == 0{
                    board.push_str(&format!("\x1b[48;5;94m{}\x1b[0m", self.tiles[file_i][rank_i]));
                }else{
                    board.push_str(&format!("\x1b[48;5;208m{}\x1b[0m", self.tiles[file_i][rank_i]));
                }
            }
            board.push('\n');
        }

        board.push_str("   a  b  c  d  e  f  g  h\n");

        write!(f, "{}", board)
    }
}