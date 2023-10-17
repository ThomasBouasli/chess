use std::fmt::Display;

use colored::Colorize;

use crate::chess::{color::Color, movement::{RelativePosition, diagonal::DiagonalMovement, line::LineMovement}};

use super::Piece;

pub struct Queen{
    color: Color,
}

impl DiagonalMovement for Queen{}

impl LineMovement for Queen{}

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
        let ( diagonal_path, diagonal_can_move) = self.diagonal_movement(position);
        let ( line_path, line_can_move) = self.line_movement(position);

        let mut movement_path = Vec::new();

        movement_path.extend(diagonal_path);
        movement_path.extend(line_path);

        (movement_path, diagonal_can_move || line_can_move)
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
