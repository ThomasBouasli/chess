use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{RelativePosition, line::LineMovement,generate_valid_moves::GenerateValidMoves}};

use super::Piece;

pub struct Rook{
    color: Color,
}

impl LineMovement for Rook{}

impl Piece for Rook{
    fn new(color: Color) -> Self {
        Rook{color}
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn value(&self) -> u8 {
        5
    }

    fn prefix(&self) -> String {
        String::from("R")
    }

    fn icon(&self) -> char{
        '♖'
    }

    fn valid_move(&self, position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.line_movement(position)
    }
}

impl GenerateValidMoves for Rook{
    fn generate_valid_moves(&self) -> Vec<RelativePosition>{
        return vec![RelativePosition {file : 1, rank : 1}];
    }
}


impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(" {} ", self.prefix())
        .color(match self.color(){
            Color::White => colored::Color::Green,
            Color::Black => colored::Color::Red,
        })
        .fmt(f)
    }
}