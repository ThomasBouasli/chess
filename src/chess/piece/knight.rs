use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::RelativePosition};

use super::Piece;

pub struct Knight{
    color: Color,
}

impl Piece for Knight{
    fn new(color: Color) -> Self {
        Knight{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        3
    }

    fn prefix(&self) -> String {
        String::from("N")
    }

    fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if (position.file.abs() == 2 && position.rank.abs() == 1) || (position.file.abs() == 1 && position.rank.abs() == 2){
            (Vec::new(), true)
        }else{
            (Vec::new(), false)
        }
    }
}

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_knight_valid_move(){
        let knight = Knight::new(Color::White);

        let (valid_moves, can_move) = knight.valid_move(&RelativePosition{file: 2, rank: 1});
        assert_eq!(valid_moves, Vec::new());
        assert_eq!(can_move, true);

        let (valid_moves, can_move) = knight.valid_move(&RelativePosition{file: 1, rank: 2});
        assert_eq!(valid_moves, Vec::new());
        assert_eq!(can_move, true);

        let (valid_moves, can_move) = knight.valid_move(&RelativePosition{file: 2, rank: 2});
        assert_eq!(valid_moves, Vec::new());
        assert_eq!(can_move, false);

        let (valid_moves, can_move) = knight.valid_move(&RelativePosition{file: 1, rank: 1});
        assert_eq!(valid_moves, Vec::new());
        assert_eq!(can_move, false);

        let (valid_moves, can_move) = knight.valid_move(&RelativePosition{file: 0, rank: 0});
        assert_eq!(valid_moves, Vec::new());
        assert_eq!(can_move, false);
    }
}