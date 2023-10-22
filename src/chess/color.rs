use std::fmt::Display;


#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Color{
    White,
    Black,
}

impl Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}