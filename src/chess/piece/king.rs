use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::RelativePosition};

use super::Piece;

pub struct King{
    color: Color,
}

impl Piece for King{
    fn new(color: Color) -> Self {
        King{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        0
    }

    fn prefix(&self) -> String {
        String::from("K")
    }

    fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if position.file.abs() <= 1 && position.rank.abs() <= 1{
            (Vec::new(), true)
        }else{
            (Vec::new(), false)
        }
    }
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}