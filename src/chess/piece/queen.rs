use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::RelativePosition};

use super::Piece;

pub struct Queen{
    color: Color,
}

impl Piece for Queen{
    fn new(color: Color) -> Self {
        Queen{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        9
    }

    fn prefix(&self) -> String {
        String::from("Q")
    }

    fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file.abs() == position.rank.abs(){
            let mut movement_path = Vec::new();
            for n in 1..=position.file.abs(){
                movement_path.push(RelativePosition{file: n * position.file.signum(), rank: n * position.rank.signum()});
            }

            (movement_path, true)
        }else if position.file.abs() == 0 || position.rank.abs() == 0{
            let mut movement_path = Vec::new();
            for n in 1..=position.file.abs(){
                movement_path.push(RelativePosition{file: n * self.multiplier(), rank: 0});
            }
            for n in 1..=position.rank.abs(){
                movement_path.push(RelativePosition{file: 0, rank: n * self.multiplier()});
            }

            (movement_path, true)
        }else{
            (Vec::new(), false)
        }
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}