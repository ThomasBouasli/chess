use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::RelativePosition};

use super::Piece;

pub struct Pawn{
    color: Color,
}

impl Piece for Pawn{
    fn new(color: Color) -> Self {
        Pawn{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        1
    }

    fn prefix(&self) -> String {
        String::from("P")
    }

    fn valid_move(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file != 0{
            (Vec::new(), false)
        }else if position.rank.abs() == 1{
            (Vec::new(), true)
        }else if position.rank.abs() == 2{
            (vec![RelativePosition {file: 0, rank: 1 * self.multiplier()}], true)
        }else{
            (Vec::new(), false)
        }
    }

    fn valid_capture(&self, position : &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file.abs() == 1 && position.rank == 1{
            (Vec::new(), true)
        }else{
            (Vec::new(), false) 
        }
    }
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}