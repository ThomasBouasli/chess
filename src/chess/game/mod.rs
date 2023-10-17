use super::{board::Board, color::Color, movement::{Movement, RelativePosition, Position}, piece::Piece};

pub mod classic;
pub struct Game{
    board: Board,
    turn: Color,
    moves: Vec<Movement>,
}

impl Game{

    pub fn board(&self) -> &Board{
        &self.board
    }
 
    pub fn turn(&self) -> &Color{
        &self.turn
    }


    pub fn move_piece(&mut self, movement: Movement) -> Result<String, String>{
        let piece = match self.board.get_tile(movement.from()).get_piece(){
            Some(piece) => piece,
            None => return Err(String::from("No piece on tile!")),
        };

        if piece.color() != self.turn(){
            return Err(String::from("Cannot move opponent's piece!"));
        }

        let relative_position = movement.to().to_relative(movement.from());

        let (path, valid) = match self.board.get_tile(movement.to()).get_piece() {
            None => self.can_move(piece, &relative_position),
            Some(to_piece) => self.can_capture(piece, to_piece, &relative_position),
        };


        if !valid{
            return Err(String::from("Invalid move!"));
        }

        if self.is_colliding(&path, movement.from()){
            return Err(String::from("Piece is colliding!"));
        }        

        
        match self.make_move(movement){
            Ok(_) => Ok(String::from("Moved!")),
            Err(e) => Err(e),
        }
    }

    fn make_move(&mut self, movement: Movement) -> Result<(), String>{
        let piece = match self.board.get_tile_mut(movement.from()).remove_piece(){
            Some(piece) => piece,
            None => return Err(String::from("No piece on tile!")),
        };

        self.board.get_tile_mut(movement.to()).set_piece(piece);

        self.turn = match self.turn(){
            Color::White => Color::Black,
            Color::Black => Color::White,
        };


        self.moves.push(movement);

        Ok(())
    }

    fn can_move(&self, piece: &Box<dyn Piece>, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool){
        piece.valid_move(relative_position)
    }

    fn can_capture(&self, piece: &Box<dyn Piece>, to_piece: &Box<dyn Piece>, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool){
        if piece.color() == to_piece.color(){
            return (Vec::new(), false);
        }

        piece.valid_capture(relative_position)
    }    

    fn is_colliding(&self, movement_path: &Vec<RelativePosition>, piece_position : &Position) -> bool{
        for relative_position in movement_path{
            let position = relative_position.to_absolute(piece_position);

            if self.board.get_tile(&position).get_piece().is_some(){
                return true;
            }
        }
        false   
    }
}

#[cfg(test)]
mod tests{
    use crate::chess::{game::classic::ClassicGame, movement::ChessNotationPosition};

    use super::*;

    #[test]
    fn test_game_can_move_piece(){
        let mut game = ClassicGame::new();

        let movement = Movement::new(Position::new(0, 1), Position::new(0, 2));

        assert_eq!(game.move_piece(movement), Ok(String::from("Moved!")));
    }

    #[test]
    fn test_game_cannot_move_opponent_piece(){
        let mut game = ClassicGame::new();

        let movement = Movement::new(Position::new(0, 6), Position::new(0, 5));

        assert_eq!(game.move_piece(movement), Err(String::from("Cannot move opponent's piece!")));
    }

    #[test]
    fn test_game_cannot_move_to_occupied_tile(){
        let mut game = ClassicGame::new();

        let movement = Movement::new(ChessNotationPosition::new('a', 1).to_position(), ChessNotationPosition::new('a', 2).to_position());

        assert_eq!(game.move_piece(movement), Err(String::from("Invalid move!")));
    }

    #[test]
    fn test_game_cannot_move_through_pieces(){
        let mut game = ClassicGame::new();

        let movement = Movement::new(ChessNotationPosition::new('a', 1).to_position(), ChessNotationPosition::new('a', 4).to_position());

        assert_eq!(game.move_piece(movement), Err(String::from("Piece is colliding!")));
    }

    #[test]
    fn test_game_can_capture_opponent_piece(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('d', 7).to_position(), ChessNotationPosition::new('d', 5).to_position());
        let capture = Movement::new(ChessNotationPosition::new('e', 4).to_position(), ChessNotationPosition::new('d', 5).to_position());

        assert_eq!(game.move_piece(m1), Ok(String::from("Moved!")));
        assert_eq!(game.move_piece(m2), Ok(String::from("Moved!")));
        assert_eq!(game.move_piece(capture), Ok(String::from("Moved!")));
    }
    
}