use crate::chess::{movement::{Movement, chess_notation::ChessNotationPosition}, game::{classic::ClassicGame, GameState}};

mod chess;

fn main() {
    let mut game = ClassicGame::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", game.board());
        println!("It's {}'s turn", game.turn());
        let movement = ask_for_movement();
        match game.move_piece(movement) {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }

        match game.state() {
            GameState::Check(_) => {
                println!("Check!");
            }
            GameState::Checkmate(color) => {
                let opposite_color = match color {
                    chess::color::Color::White => chess::color::Color::Black,
                    chess::color::Color::Black => chess::color::Color::White,
                };

                println!("Checkmate! {} wins!", opposite_color);
                break;
            }
            GameState::Stalemate => {
                println!("Stalemate!");
                break;
            }
            _ => (),
        }
    }


}


fn ask_for_movement() -> Movement{
    let mut movement = String::new();
    println!("Please enter your movement: ");
    std::io::stdin().read_line(&mut movement).expect("Failed to read line");
    let movement = movement.trim();
    
    let from = match ChessNotationPosition::from_str(&movement[0..2]) {
        Ok(position) => position,
        Err(error) => {
            println!("{}", error);
            return ask_for_movement();
        }
    };

    let to = match ChessNotationPosition::from_str(&movement[2..4]) {
        Ok(position) => position,
        Err(error) => {
            println!("{}", error);
            return ask_for_movement();
        }
    };

    Movement::new(from.to_position(), to.to_position())
}