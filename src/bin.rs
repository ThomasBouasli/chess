use crate::chess::{movement::{Movement, self}, game::classic::ClassicGame};



mod chess;

fn main() {
    let mut game = ClassicGame::new();

    println!("{}", game.board());

    
    let moves = vec!(
        Movement::new(movement::ChessNotationPosition::new('e', 2).to_position(), movement::ChessNotationPosition::new('e', 4).to_position()),
        Movement::new(movement::ChessNotationPosition::new('e', 7).to_position(), movement::ChessNotationPosition::new('e', 5).to_position()),
        Movement::new(movement::ChessNotationPosition::new('g', 1).to_position(), movement::ChessNotationPosition::new('f', 3).to_position()),
        Movement::new(movement::ChessNotationPosition::new('b', 8).to_position(), movement::ChessNotationPosition::new('c', 6).to_position()),
        Movement::new(movement::ChessNotationPosition::new('f', 1).to_position(), movement::ChessNotationPosition::new('c', 4).to_position()),
        Movement::new(movement::ChessNotationPosition::new('d', 7).to_position(), movement::ChessNotationPosition::new('d', 6).to_position()),
        Movement::new(movement::ChessNotationPosition::new('c', 4).to_position(), movement::ChessNotationPosition::new('f', 7).to_position()),
        Movement::new(movement::ChessNotationPosition::new('e', 8).to_position(), movement::ChessNotationPosition::new('f', 7).to_position()),
        Movement::new(movement::ChessNotationPosition::new('d', 2).to_position(), movement::ChessNotationPosition::new('d', 3).to_position()),
        Movement::new(movement::ChessNotationPosition::new('f', 7).to_position(), movement::ChessNotationPosition::new('f', 6).to_position()),
        Movement::new(movement::ChessNotationPosition::new('c', 1).to_position(), movement::ChessNotationPosition::new('g', 5).to_position()),
        Movement::new(movement::ChessNotationPosition::new('f', 6).to_position(), movement::ChessNotationPosition::new('f', 5).to_position()),
    );

    for m in moves{
        match game.move_piece(m){
            Err(e) => println!("Error: {}", e),
            _ => (),
        }

        println!("{}", game.board());
    }

}
