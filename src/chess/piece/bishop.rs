use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::RelativePosition};

use super::Piece;

pub struct Bishop{
    color: Color,
}

impl Piece for Bishop{
    fn new(color: Color) -> Self {
        Bishop{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        3
    }

    fn prefix(&self) -> String {
        String::from("B")
    }

    fn valid_move(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file.abs() == position.rank.abs(){
            let mut movement_path = Vec::new();
            for n in 1..position.file {
                let relative = RelativePosition{file: n * position.file.signum(), rank: n * position.rank.signum() * self.multiplier()};
                movement_path.push(relative);
            }

            (movement_path, true)
        }else{
            (Vec::new(), false)
        }
    }
}

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}