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