use super::{board::Board, color::Color, movement::{Movement, self, ChessNotationPosition, RelativePosition}, piece::{pawn::Pawn, king::King, Piece, bishop::Bishop, knight::Knight, rook::Rook, queen::Queen}};

pub struct Game{
    board: Board,
    turn: Color,
    moves: Vec<Movement>,
    // in_check: Option<Color>,
}

impl Game{
    pub fn new() -> Self{

        let mut board = Board::new();

        for n in 0..8{
            board.set_piece(Box::new(Pawn::new(Color::White)), ChessNotationPosition::new((n as u8 + 97) as char, 2).to_position());
            board.set_piece(Box::new(Pawn::new(Color::Black)), ChessNotationPosition::new((n as u8 + 97) as char, 7).to_position());
        }
    
        board.set_piece(Box::new(Rook::new(Color::White)), ChessNotationPosition::new('a', 1).to_position());
        board.set_piece(Box::new(Knight::new(Color::White)), ChessNotationPosition::new('b', 1).to_position());
        board.set_piece(Box::new(Bishop::new(Color::White)), ChessNotationPosition::new('c', 1).to_position());
        board.set_piece(Box::new(Queen::new(Color::White)), ChessNotationPosition::new('d', 1).to_position());
        board.set_piece(Box::new(King::new(Color::White)), ChessNotationPosition::new('e', 1).to_position());
        board.set_piece(Box::new(Bishop::new(Color::White)), ChessNotationPosition::new('f', 1).to_position());
        board.set_piece(Box::new(Knight::new(Color::White)), ChessNotationPosition::new('g', 1).to_position());
        board.set_piece(Box::new(Rook::new(Color::White)), ChessNotationPosition::new('h', 1).to_position());


        board.set_piece(Box::new(Rook::new(Color::Black)), ChessNotationPosition::new('a', 8).to_position());
        board.set_piece(Box::new(Knight::new(Color::Black)), ChessNotationPosition::new('b', 8).to_position());
        board.set_piece(Box::new(Bishop::new(Color::Black)), ChessNotationPosition::new('c', 8).to_position());
        board.set_piece(Box::new(Queen::new(Color::Black)), ChessNotationPosition::new('d', 8).to_position());
        board.set_piece(Box::new(King::new(Color::Black)), ChessNotationPosition::new('e', 8).to_position());
        board.set_piece(Box::new(Bishop::new(Color::Black)), ChessNotationPosition::new('f', 8).to_position());
        board.set_piece(Box::new(Knight::new(Color::Black)), ChessNotationPosition::new('g', 8).to_position());
        board.set_piece(Box::new(Rook::new(Color::Black)), ChessNotationPosition::new('h', 8).to_position());

    

        Self{
            board,
            turn: Color::White,
            moves: Vec::new(),
            // in_check: None,
        }
    }

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

        let chess_notation = format!("{}{}", piece.prefix(), movement.to());

        if piece.color() != self.turn(){
            return Err(String::from("Cannot move opponent's piece!"));
        }


        let relative_position = RelativePosition {
            file: (movement.to().file as i8 - movement.from().file as i8),
            rank: movement.to().rank as i8 - movement.from().rank as i8,
        };

        let (movement_path, valid_movement) = piece.valid_move(&relative_position);

        if let Some(to_piece) = self.board.get_tile(movement.to()).get_piece(){
            if to_piece.color() == self.turn(){
                return Err(String::from("Cannot capture own piece!"));
            }else{
                let (capture_path, valid_capture) = piece.valid_capture(&relative_position);

                if !valid_capture{
                    return Err(String::from("Invalid capture!"));
                }
                for relative_position in capture_path{
                    let position = relative_position.to_absolute(&movement.from());

                    if self.board.get_tile(&position).get_piece().is_some(){
                        return Err(String::from("Cannot move through pieces!"));
                    }
                }
            }
        }

        if !valid_movement{
            return Err(String::from("Invalid move!"));
        }


        for relative_position in movement_path{
            let position = relative_position.to_absolute(&movement.from());

            if self.board.get_tile(&position).get_piece().is_some(){
                return Err(String::from("Cannot move through pieces!"));
            }
        }

        let piece = self.board.get_tile_mut(movement.from()).remove_piece().unwrap();
        self.board.get_tile_mut(movement.to()).set_piece(piece);

        self.turn = match self.turn(){
            Color::White => Color::Black,
            Color::Black => Color::White,
        };


        self.moves.push(movement);
        

        return Ok(chess_notation);
    }
}