use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{RelativePosition, diagonal::DiagonalMovement, generate_valid_moves::GenerateValidMoves}};

use super::Piece;

pub struct Bishop{
    color: Color,
}

impl DiagonalMovement for Bishop{}

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

    fn icon(&self) -> char{
        '♗'
    }

    fn valid_move(&self,  position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        self.diagonal_movement(position)
    }
}

impl GenerateValidMoves for Bishop{
    fn generate_valid_moves(&self) -> Vec<RelativePosition>{
        return vec![RelativePosition {file : 1, rank : 1}];
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