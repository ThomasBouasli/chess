use std::fmt::Display;

use super::{color::Color, movement::{RelativePosition, generate_valid_moves::GenerateValidMoves}};

pub mod pawn;
pub mod king;
pub mod bishop;
pub mod knight;
pub mod rook;
pub mod queen;

pub trait Piece : Display + GenerateValidMoves{
    fn new(color: Color) -> Self where Self: Sized;

    fn value(&self) -> u8;

    fn color(&self) -> &Color;

    fn icon(&self) -> char;

    fn prefix(&self) -> String;

    fn valid_move(&self, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool);

    fn valid_capture(&self, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool){
        self.valid_move(relative_position)
    }

    fn moved(&mut self){}

    fn multiplier(&self) -> i8 {
        match self.color(){
            Color::White => 1,
            Color::Black => -1,
        }
    }
}