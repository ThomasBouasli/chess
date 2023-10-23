use super::{board::Board, color::Color, movement::{Movement, relative_position::RelativePosition, absolute_position::AbsolutePosition, generate_valid_moves::GenerateValidMoves}, piece::Piece};

pub mod classic;


#[derive(Clone, PartialEq, Debug)]
pub enum GameState{
    Check(Color),
    Checkmate(Color),
    Stalemate,
    InProgress,
    Promoting,
}

#[derive(Clone)]
pub struct Game{
    board: Board,
    turn: Color,
    moves: Vec<Movement>,
    state: GameState
}

pub trait IGame{
    fn new(board: Board, turn: Color) -> Self;
    fn board(&self) -> &Board;
    fn turn(&self) -> &Color;
    fn state(&self) -> &GameState;
    fn move_piece(&mut self, movement: Movement) -> Result<&GameState, String>;
    fn get_values(&self) -> (i32, i32);
}

impl Game{

    pub fn new(board: Board, turn: Color) -> Self{
        Game{
            board,
            turn,
            moves: Vec::new(),
            state: GameState::InProgress,
        }
    }

    pub fn board(&self) -> &Board{
        &self.board
    }
 
    pub fn turn(&self) -> &Color{
        &self.turn
    }
    

    /// Returns the current state of the game
    /// 
    /// ### Returns
    /// 
    /// * `GameState::Check(color)` - If the current player is in check
    /// * `GameState::Checkmate(color)` - If the current player is in checkmate
    /// * `GameState::Stalemate` - If the game is in stalemate
    /// * `GameState::InProgress` - If the game is in progress
    /// 
    pub fn state(&self) -> &GameState{
        &self.state
    }


    /// Moves a piece on the board
    /// 
    /// ### Arguments
    /// 
    /// * `movement` - The movement to be made
    /// 
    /// ### Returns
    /// 
    /// Returns a Result containing a reference to the game state if the move was successful, otherwise an error message
    /// 
    /// ## Examples
    /// 
    /// ```
    /// use chess::{game::classic::ClassicGame, movement::chess_notation::ChessNotationPosition};
    /// 
    /// let mut game = ClassicGame::new();
    /// 
    /// let movement = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
    /// 
    /// let result = match game.move_piece(movement){
    ///    Ok(state) => state,
    ///    Err(e) => panic!(e),
    /// }; // GameState::InProgress
    /// 
    /// do_something_fun_with(result);
    /// 
    /// ```
    pub fn move_piece(&mut self, movement: Movement) -> Result<&GameState, String>{
        if self.state == GameState::Promoting{
            return Err(String::from("Cannot move piece while promoting!"));
        }

        self.state = GameState::InProgress;

        if let Err(e) = self.is_legal_play(&movement){
            return Err(e);
        }


        let mut game = self.clone();

        game.make_move(movement.clone())?;

        if let Some(color) = game.is_check(){
            if color == *self.turn(){
                return Err(String::from("Cannot move into check!"));
            }

            self.state = GameState::Check(color);
        }

        if let Some(state) = game.is_check_mate(){
            self.state = state;
        }
        
        match self.make_move(movement){
            Ok(_) => Ok(&self.state),
            Err(e) => Err(e),
        }
    }

    /// Returns the value of the pieces on the board for each player
    /// 
    /// ### Returns
    /// 
    /// * `(white_value, black_value)` - The value of the pieces for each player
    pub fn get_values(&self) -> (i32, i32){
        let mut white_value : i32 = 0;
        let mut black_value : i32 = 0;

        for (tile, _) in self.board.get_tiles(){
            if let Some(piece) = tile.get_piece(){
                if piece.color() == &Color::White{
                    white_value += piece.value() as i32;
                }else{
                    black_value += piece.value() as i32;
                }
            }
        }

        (white_value, black_value)
    }

    fn make_move(&mut self, movement: Movement) -> Result<(), String>{
        
        match self.execute_castle(&movement) {
            Ok(_) => (),
            Err(err) => {
                if err != String::from("Movement is not a castle!"){
                    return Err(err);
                }
            }
        };


        let mut piece = match self.board.get_tile_mut(movement.from()).remove_piece(){
            Some(piece) => piece,
            None => return Err(String::from("No piece on tile!")),
        };

        piece.moved();

        self.board.get_tile_mut(movement.to()).set_piece(piece);

        self.turn = match self.turn(){
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.moves.push(movement);

        Ok(())
    }

    fn is_legal_play(&self, movement: &Movement) -> Result<(), String>{
        let piece = match self.board.get_tile(movement.from()).get_piece(){
            Some(piece) => piece,
            None => return Err(String::from("No piece on tile!")),
        };

        if piece.color() != self.turn(){
            return Err(String::from("Cannot move opponent's piece!"));
        }

        let relative_position = movement.to().to_relative(movement.from());

        let valid = match self.board.get_tile(movement.to()).get_piece() {
            None => self.is_legal_move(piece, movement.from(), &relative_position),
            Some(to_piece) => self.is_legal_capture(piece, movement.from(), to_piece, &relative_position),
        };

        if valid {
            return Ok(());
        }

        if piece.prefix() == 'P'{
            return self.allow_en_passante(piece, movement);
        }

        if piece.prefix() == 'K'{
            return self.allow_castle(piece, movement);
        }

        Err(String::from("Invalid move!"))
    }

    fn allow_en_passante(&self, piece : &Piece, movement: &Movement) -> Result<(), String>{
        if piece.prefix() != 'P'{
            return Err(String::from("Invalid move!"));
        }

        let last_move = match self.moves.last() {
            Some(movement) => movement,
            None => return Err(String::from("Invalid move!")),
        };

        let last_move_piece = self.board.get_tile(last_move.to()).get_piece().as_ref().unwrap();

        if last_move_piece.prefix() != 'P' {
            return Err(String::from("Invalid move!"));
        }

        if last_move.from().to_relative(last_move.to()) != RelativePosition::new(0, 2){
            return Err(String::from("Invalid move!"));
        }

        if  RelativePosition::new(0, 1).to_absolute(last_move.to()).unwrap() != *movement.to(){
            return Err(String::from("Invalid move!"));
        }

        return Ok(());
    }

    /// Allows the player to castle
    /// 
    /// ### Arguments
    /// 
    /// * `piece` - The king piece
    /// * `movement` - The movement to be made
    /// 
    /// A queen side castle is defined by moving the king two tiles to the left
    /// A king side castle is defined by moving the king two tiles to the right
    /// 
    /// then a check is ran to verify that the king and corresponding rook have not moved
    /// also checks that the king is not moving through check
    fn allow_castle(&self, piece : &Piece, movement: &Movement) -> Result<(), String>{


        if piece.prefix() != 'K'{
            return Err(String::from("Invalid move!"));
        }

        if piece.has_moved(){
            return Err(String::from("Invalid move!"));
        }

        enum Castle{
            QueenSide,
            KingSide,
        }

        let allowed_rank = match piece.color(){
            Color::White => 0,
            Color::Black => 7,
        };

        let allowed_file = 4;

        let allowed_king_position = AbsolutePosition::new(allowed_file, allowed_rank);

        if movement.from() != &allowed_king_position{
            return Err(String::from("Invalid move!"));
        }

        let castle = match movement.to().file{
            2 => Castle::QueenSide,
            6 => Castle::KingSide,
            _ => return Err(String::from("Invalid move!")),
        };

        let rook_position = match castle{
            Castle::QueenSide => AbsolutePosition::new(0, allowed_rank),
            Castle::KingSide => AbsolutePosition::new(7, allowed_rank),
        };

        let rook = match self.board.get_tile(&rook_position).get_piece(){
            Some(piece) => {
                if piece.prefix() != 'R'{
                    return Err(String::from("Invalid move!"));
                }

                piece
            },
            None => return Err(String::from("Invalid move!"))
        };

        if rook.has_moved(){
            return Err(String::from("Invalid move!"));
        }

        let relative_position = movement.to().to_relative(movement.from());

        let (path, valid) = match castle{
            Castle::QueenSide => piece.castle_queen_side(&relative_position),
            Castle::KingSide => piece.castle_king_side(&relative_position),
        };

        if !valid || self.is_colliding(&path, movement.from()){
            return Err(String::from("Invalid move!"));
        }

        if self.path_is_in_check(&path, movement.from()){
            return Err(String::from("Invalid move!"));
        }

        Ok(())
    }

    fn execute_castle(&mut self, movement: &Movement) -> Result<(), String>{

        enum Castle{
            QueenSide,
            KingSide,
        }

        let piece = match self.board.get_tile(movement.from()).get_piece(){
            Some(piece) => piece,
            None => return Err(String::from("Movement is not a castle!")),
        };

        if !self.movement_is_castle(piece, movement){
            return Err(String::from("Movement is not a castle!"));
        }

        let castle = match movement.to().file{
            2 => Castle::QueenSide,
            6 => Castle::KingSide,
            _ => return Err(String::from("Invalid move!")),
        };

        let rook_position = match castle{
            Castle::QueenSide => AbsolutePosition::new(0, movement.to().rank),
            Castle::KingSide => AbsolutePosition::new(7, movement.to().rank),
        };

        let rook = match self.board.get_tile_mut(&rook_position).remove_piece(){
            Some(piece) => piece,
            None => return Err(String::from("Invalid move!")),
        };

        let rook_position = match castle{
            Castle::QueenSide => AbsolutePosition::new(3, movement.to().rank),
            Castle::KingSide => AbsolutePosition::new(5, movement.to().rank),
        };

        self.board.get_tile_mut(&rook_position).set_piece(rook);

        Ok(())
    }

    fn movement_is_castle(&self, piece : &Piece, movement: &Movement) -> bool{
        if piece.prefix() != 'K'{
            return false;
        }

        let relative_position = movement.to().to_relative(movement.from());

        if !piece.castle_king_side(&relative_position).1 && !piece.castle_queen_side(&relative_position).1{
            return false;
        }

        return true;
    }

    fn is_legal_move(&self, piece: &Piece, piece_position: &AbsolutePosition, relative_position: &RelativePosition) -> bool{
        let (path, valid) = piece.valid_move(relative_position);

        if !valid || self.is_colliding(&path, piece_position){
            return false;
        }

        true
    }

    fn is_legal_capture(&self, piece: &Piece, piece_position: &AbsolutePosition, to_piece: &Piece, relative_position: &RelativePosition) -> bool{
        if piece.color() == to_piece.color(){
            return false;
        }

        let (path, valid) = piece.valid_capture(relative_position);

        if !valid || self.is_colliding(&path, piece_position){
            return false;
        }

        true
    }
    
    fn movement_from_chess_notation(&self, destination: AbsolutePosition, piece_prefix : Option<char>, rank_from : Option<usize>, file_from: Option<usize>, is_capture: bool) -> Result<Movement,String>{
        let piece_prefix = match piece_prefix{
            Some(prefix) => prefix,
            None => 'P',
        };

        let mut response : Option<Movement> = None;

        let legal_moves = match is_capture{
            true => self.generate_legal_captures_for_piece_type(piece_prefix, &self.turn()),
            false => self.generate_legal_move_for_piece_type(piece_prefix, &self.turn()),
        };

        for movement in legal_moves{
            if movement.to() == &destination{
                if response.is_some(){

                    if let Some(rank_from) = rank_from{
                        if movement.from().rank != rank_from{
                            continue;
                        }
                    }

                    if let Some(file_from) = file_from{
                        if movement.from().file != file_from{
                            continue;
                        }
                    }

                    return Err(String::from("Ambiguous move!"));

                }

                response = Some(movement);
            }
        }

        match response{
            Some(movement) => Ok(movement),
            None => Err(String::from("Invalid move!")),
        }
    }

    fn generate_legal_move_for_piece_type(&self, piece_prefix: char,color : &Color) -> Vec<Movement>{
        let mut legal_moves = Vec::new();

        for (tile, position) in self.board.get_tiles(){
            if let Some(piece) = tile.get_piece(){
                if piece.color() == color{
                    if piece.prefix() != piece_prefix {
                        continue;
                    }
                    for relative_position in piece.generate_valid_moves(){

                        let absolute = match relative_position.to_absolute(&position){
                            Ok(position) => position,
                            Err(_) => continue,
                        };

                        match self.is_legal_play(&Movement::new(position.clone(), absolute.clone())){
                            Ok(_) => (),
                            Err(_) => continue,
                        };

                        let movement = Movement::new(position.clone(), absolute);

                        let mut game = self.clone();

                        game.make_move(movement.clone()).unwrap();

                        if let Some(color) = game.is_check(){
                            if color == *self.turn(){
                                continue;
                            }
                        }

                        legal_moves.push(movement);
                    }
                }
            }
        }

        legal_moves
    }


    fn generate_legal_captures_for_piece_type(&self, piece_prefix: char,color : &Color) -> Vec<Movement>{
        let mut legal_moves = Vec::new();

        for (tile, position) in self.board.get_tiles(){
            if let Some(piece) = tile.get_piece(){
                if piece.color() == color{
                    if piece.prefix() != piece_prefix {
                        continue;
                    }
                    for relative_position in piece.generate_valid_captures(){

                        let absolute = match relative_position.to_absolute(&position){
                            Ok(position) => position,
                            Err(_) => continue,
                        };

                        match self.is_legal_play(&Movement::new(position.clone(), absolute.clone())){
                            Ok(_) => (),
                            Err(_) => continue,
                        };

                        let movement = Movement::new(position.clone(), absolute);

                        let mut game = self.clone();

                        game.make_move(movement.clone()).unwrap();

                        if let Some(color) = game.is_check(){
                            if color == *self.turn(){
                                continue;
                            }
                        }

                        legal_moves.push(movement);
                    }
                }
            }
        }

        legal_moves
    }

    fn generate_legal_plays(&self, color : &Color) -> Vec<Movement>{
        let mut legal_moves = Vec::new();

        for (tile, position) in self.board.get_tiles(){
            if let Some(piece) = tile.get_piece(){
                if piece.color() == color{
                    for relative_position in piece.generate_valid_plays(){

                        let absolute = match relative_position.to_absolute(&position){
                            Ok(position) => position,
                            Err(_) => continue,
                        };

                        match self.is_legal_play(&Movement::new(position.clone(), absolute.clone())){
                            Ok(_) => (),
                            Err(_) => continue,
                        };

                        let movement = Movement::new(position.clone(), absolute);

                        let mut game = self.clone();

                        game.make_move(movement.clone()).unwrap();

                        if let Some(color) = game.is_check(){
                            if color == *self.turn(){
                                continue;
                            }
                        }

                        legal_moves.push(movement);
                    }
                }
            }
        }

        legal_moves
    }

    fn is_colliding(&self, movement_path: &Vec<RelativePosition>, piece_position : &AbsolutePosition) -> bool{
        for relative_position in movement_path{
            let position = relative_position.to_absolute(piece_position).unwrap();

            if self.board.get_tile(&position).get_piece().is_some(){
                return true;
            }
        }
        false   
    }

    fn path_is_in_check(&self, movement_path: &Vec<RelativePosition>, piece_position : &AbsolutePosition) -> bool{
        let game = self.clone();

        for relative_position in movement_path{
            let position = relative_position.to_absolute(piece_position).unwrap();

            let mut game = game.clone();

            game.make_move(Movement::new(piece_position.clone(), position.clone())).unwrap();

            if let Some(color) = game.is_check(){
                if color == *self.turn(){
                    return true;
                }
            }
        }

        false
    }

    fn is_check_mate(&self) -> Option<GameState>{
        let legal_moves = self.generate_legal_plays(&self.turn());

        if legal_moves.len() == 0{
            if let Some(color) = self.is_check(){
                return Some(GameState::Checkmate(color));
            }

            return Some(GameState::Stalemate);
        }

        None
    }

    fn is_check(&self) -> Option<Color>{
        if self.is_check_color(&Color::White){
            Some(Color::White)
        }else if self.is_check_color(&Color::Black){
            Some(Color::Black)
        }else{
            None
        }
    }


    fn is_check_color(&self, color: &Color) -> bool{
        let king_position = match self.board.get_king_position(color){
            Some(position) => position,
            None => return false,
        };

        let king_piece = match self.board.get_tile(&king_position).get_piece(){
            Some(piece) => piece,
            None => return false,
        };

        for (tile, position) in self.board.get_tiles(){
            if let Some(piece) = tile.get_piece(){
                if piece.color() != color{
                    let relative_position = king_position.to_relative(&position);

                    let valid = self.is_legal_capture(piece, &position, king_piece, &relative_position);

                    if valid {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests{
    use crate::chess::{game::classic::ClassicGame, movement::chess_notation::ChessNotationPosition};

    use super::*;

    #[test]
    fn test_game_can_move_piece(){
        let mut game = ClassicGame::new();

        let movement = Movement::new(AbsolutePosition::new(0, 1), AbsolutePosition::new(0, 2));

        assert_eq!(game.move_piece(movement), Ok(&GameState::InProgress));
    }

    #[test] 
    fn test_game_cannot_move_opponent_piece(){
        let mut game = ClassicGame::new();

        let movement = Movement::new(AbsolutePosition::new(0, 6), AbsolutePosition::new(0, 5));

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

        assert_eq!(game.move_piece(movement), Err(String::from("Invalid move!")));
    }

    #[test]
    fn test_game_can_capture_opponent_piece(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('d', 7).to_position(), ChessNotationPosition::new('d', 5).to_position());
        let capture = Movement::new(ChessNotationPosition::new('e', 4).to_position(), ChessNotationPosition::new('d', 5).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(capture), Ok(&GameState::InProgress));
    }

    #[test]
    fn test_en_passante_is_valid(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('d', 7).to_position(), ChessNotationPosition::new('d', 5).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('e', 4).to_position(), ChessNotationPosition::new('e', 5).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('f', 7).to_position(), ChessNotationPosition::new('f', 5).to_position());
        let m5 = Movement::new(ChessNotationPosition::new('e', 5).to_position(), ChessNotationPosition::new('f', 6).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        println!("{}", game.board());
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        println!("{}", game.board());
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        println!("{}", game.board());
        assert_eq!(game.move_piece(m4), Ok(&GameState::InProgress));
        println!("{}", game.board());
        assert_eq!(game.move_piece(m5), Ok(&GameState::InProgress));
        println!("{}", game.board());
    }

    #[test]
    fn test_generates_valid_moves(){
        let game = ClassicGame::new();

        let moves = game.generate_legal_plays(game.turn());

        for movement in moves{
            let mut game = game.clone();

            println!("Movement {:?}", movement);
            println!("{}", game.board());
            assert_eq!(game.move_piece(movement), Ok(&GameState::InProgress));
        }
    }

    #[test]
    fn test_generates_valid_moves_and_blocks_checks(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('f', 7).to_position(), ChessNotationPosition::new('f', 5).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('d', 1).to_position(), ChessNotationPosition::new('h', 5).to_position());


        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3),  Ok(&GameState::Check(Color::Black)));
        assert_eq!(game.state(), &GameState::Check(Color::Black));

        let moves = game.generate_legal_plays(game.turn());

        let expected_move = Movement::new(ChessNotationPosition::new('g', 7).to_position(), ChessNotationPosition::new('g', 6).to_position());

        assert_eq!(moves.contains(&expected_move), true);
        assert_eq!(moves.len(), 1);
    }
    

    #[test]
    fn test_checks_are_detected(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('e', 7).to_position(), ChessNotationPosition::new('e', 5).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('f', 1).to_position(), ChessNotationPosition::new('c', 4).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('d', 8).to_position(), ChessNotationPosition::new('h', 4).to_position());
        let m5 = Movement::new(ChessNotationPosition::new('c', 4).to_position(), ChessNotationPosition::new('b', 5).to_position());
        let m6 = Movement::new(ChessNotationPosition::new('h', 4).to_position(), ChessNotationPosition::new('e', 4).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m4), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m5), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m6),  Ok(&GameState::Check(Color::White)));


        assert_eq!(game.state(), &GameState::Check(Color::White));
    }

    #[test]
    fn test_cannot_make_a_move_that_keeps_you_in_check(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('e', 7).to_position(), ChessNotationPosition::new('e', 5).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('f', 1).to_position(), ChessNotationPosition::new('c', 4).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('d', 8).to_position(), ChessNotationPosition::new('h', 4).to_position());
        let m5 = Movement::new(ChessNotationPosition::new('c', 4).to_position(), ChessNotationPosition::new('b', 5).to_position());
        let m6 = Movement::new(ChessNotationPosition::new('h', 4).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m7 = Movement::new(ChessNotationPosition::new('b', 5).to_position(), ChessNotationPosition::new('c', 6).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m4), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m5), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m6),  Ok(&GameState::Check(Color::White)));
        assert_eq!(game.move_piece(m7), Err(String::from("Cannot move into check!")));
    }

    #[test]
    fn test_can_detect_check_mate(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('f', 2).to_position(), ChessNotationPosition::new('f', 3).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('e', 7).to_position(), ChessNotationPosition::new('e', 5).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('g', 2).to_position(), ChessNotationPosition::new('g', 4).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('d', 8).to_position(), ChessNotationPosition::new('h', 4).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m4), Ok(&GameState::Checkmate(Color::White)));


        assert_eq!(game.state(), &GameState::Checkmate(Color::White));
    }

    #[test]
    fn test_can_detect_stalemate(){
        let mut game = ClassicGame::new();

        let mut moves = Vec::new();

        moves.push(Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 3).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('a', 7).to_position(), ChessNotationPosition::new('a', 5).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('d', 1).to_position(), ChessNotationPosition::new('h', 5).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('a', 8).to_position(), ChessNotationPosition::new('a', 6).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('h', 5).to_position(), ChessNotationPosition::new('a', 5).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('h', 7).to_position(), ChessNotationPosition::new('h', 5).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('h', 2).to_position(), ChessNotationPosition::new('h', 4).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('a', 6).to_position(), ChessNotationPosition::new('h', 6).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('a', 5).to_position(), ChessNotationPosition::new('c', 7).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('f', 7).to_position(), ChessNotationPosition::new('f', 6).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('c', 7).to_position(), ChessNotationPosition::new('d', 7).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('e', 8).to_position(), ChessNotationPosition::new('f', 7).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('d', 7).to_position(), ChessNotationPosition::new('b', 7).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('d', 8).to_position(), ChessNotationPosition::new('d', 3).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('b', 7).to_position(), ChessNotationPosition::new('b', 8).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('d', 3).to_position(), ChessNotationPosition::new('h', 7).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('b', 8).to_position(), ChessNotationPosition::new('c', 8).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('f', 7).to_position(), ChessNotationPosition::new('g', 6).to_position()));
        moves.push(Movement::new(ChessNotationPosition::new('c', 8).to_position(), ChessNotationPosition::new('e', 6).to_position()));

        for (i, movement) in moves.iter().enumerate(){
            if i == moves.len() - 1{
                let state = game.state().clone();
                assert_eq!(game.move_piece(movement.clone()), Ok(&GameState::Stalemate), "State is not stalemate, it is {:?}", state);
            }else{
                game.move_piece(movement.clone()).unwrap();
            }
        }
    }

    #[test]
    fn test_can_get_accurate_values(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('f', 7).to_position(), ChessNotationPosition::new('f', 5).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('e', 4).to_position(), ChessNotationPosition::new('f', 5).to_position());

        assert_eq!(game.get_values(), (39, 39));


        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.get_values(), (39, 39));

        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.get_values(), (39, 39));

        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.get_values(), (39, 38));
    }

    #[test]
    fn test_can_castle_king_side(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('a', 7).to_position(), ChessNotationPosition::new('a', 6).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('f', 1).to_position(), ChessNotationPosition::new('c', 4).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('b', 7).to_position(), ChessNotationPosition::new('b', 6).to_position());
        let m5 = Movement::new(ChessNotationPosition::new('g', 1).to_position(), ChessNotationPosition::new('f', 3).to_position());
        let m6 = Movement::new(ChessNotationPosition::new('c', 7).to_position(), ChessNotationPosition::new('c', 6).to_position());
        let m7 = Movement::new(ChessNotationPosition::new('e', 1).to_position(), ChessNotationPosition::new('g', 1).to_position());


        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m4), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m5), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m6), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m7), Ok(&GameState::InProgress));

        let king_tile = game.board.get_tile(&ChessNotationPosition::new('g', 1).to_position());
        let rook_tile = game.board.get_tile(&ChessNotationPosition::new('f', 1).to_position());

        let king_piece = match king_tile.get_piece(){
            Some(piece) => piece,
            None => panic!("No piece on tile!"),
        };

        let rook_piece = match rook_tile.get_piece(){
            Some(piece) => piece,
            None => panic!("No piece on tile!"),
        };

        assert_eq!(king_piece.prefix(), 'K');
        assert_eq!(rook_piece.prefix(), 'R');
    }

    #[test]
    fn test_can_castle_queen_side(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('d', 2).to_position(), ChessNotationPosition::new('d', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('a', 7).to_position(), ChessNotationPosition::new('a', 6).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('c', 1).to_position(), ChessNotationPosition::new('g', 5).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('b', 7).to_position(), ChessNotationPosition::new('b', 6).to_position());
        let m5 = Movement::new(ChessNotationPosition::new('b', 1).to_position(), ChessNotationPosition::new('c', 3).to_position());
        let m6 = Movement::new(ChessNotationPosition::new('c', 7).to_position(), ChessNotationPosition::new('c', 6).to_position());
        let m7 = Movement::new(ChessNotationPosition::new('d', 1).to_position(), ChessNotationPosition::new('d', 2).to_position());
        let m8 = Movement::new(ChessNotationPosition::new('d', 7).to_position(), ChessNotationPosition::new('d', 6).to_position());
        let m9 = Movement::new(ChessNotationPosition::new('e', 1).to_position(), ChessNotationPosition::new('c', 1).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m4), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m5), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m6), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m7), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m8), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m9), Ok(&GameState::InProgress));

        let king_tile = game.board.get_tile(&ChessNotationPosition::new('c', 1).to_position());
        let rook_tile = game.board.get_tile(&ChessNotationPosition::new('d', 1).to_position());

        let king_piece = match king_tile.get_piece(){
            Some(piece) => piece,
            None => panic!("No piece on tile!"),
        };

        let rook_piece = match rook_tile.get_piece(){
            Some(piece) => piece,
            None => panic!("No piece on tile!"),
        };

        assert_eq!(king_piece.prefix(), 'K');
        assert_eq!(rook_piece.prefix(), 'R');
    }

    #[test]
    fn test_cant_castle_queen_side_when_path_is_in_check(){
        let mut game = ClassicGame::new();

        let m1 = Movement::new(ChessNotationPosition::new('d', 2).to_position(), ChessNotationPosition::new('d', 4).to_position());
        let m2 = Movement::new(ChessNotationPosition::new('g', 7).to_position(), ChessNotationPosition::new('g', 6).to_position());
        let m3 = Movement::new(ChessNotationPosition::new('c', 1).to_position(), ChessNotationPosition::new('g', 5).to_position());
        let m4 = Movement::new(ChessNotationPosition::new('f', 8).to_position(), ChessNotationPosition::new('h', 6).to_position());
        let m5 = Movement::new(ChessNotationPosition::new('b', 1).to_position(), ChessNotationPosition::new('c', 3).to_position());
        let m6 = Movement::new(ChessNotationPosition::new('h', 6).to_position(), ChessNotationPosition::new('g', 5).to_position());
        let m7 = Movement::new(ChessNotationPosition::new('d', 1).to_position(), ChessNotationPosition::new('d', 3).to_position());
        let m8 = Movement::new(ChessNotationPosition::new('d', 7).to_position(), ChessNotationPosition::new('d', 6).to_position());
        let m9 = Movement::new(ChessNotationPosition::new('e', 1).to_position(), ChessNotationPosition::new('c', 1).to_position());

        assert_eq!(game.move_piece(m1), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m2), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m3), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m4), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m5), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m6), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m7), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m8), Ok(&GameState::InProgress));
        assert_eq!(game.move_piece(m9), Err(String::from("Invalid move!")));

        let king_tile = game.board.get_tile(&ChessNotationPosition::new('e', 1).to_position());
        let rook_tile = game.board.get_tile(&ChessNotationPosition::new('a', 1).to_position());

        let king_piece = match king_tile.get_piece(){
            Some(piece) => piece,
            None => panic!("No piece on tile!"),
        };

        let rook_piece = match rook_tile.get_piece(){
            Some(piece) => piece,
            None => panic!("No piece on tile!"),
        };

        assert_eq!(king_piece.prefix(), 'K');
        assert_eq!(rook_piece.prefix(), 'R');
    }

    #[test]
    fn test_should_find_move_from_chess_notation(){
        let game = ClassicGame::new();

        let input = ChessNotationPosition::new('e', 4).to_position();

        let movement = game.movement_from_chess_notation(input, None, None, None, false);

        assert_eq!(movement, Ok(Movement::new(ChessNotationPosition::new('e', 2).to_position(), ChessNotationPosition::new('e', 4).to_position())));
    }

    #[test]
    fn test_should_find_move_from_chess_notation_in_ambiguous_case(){
        let mut game = ClassicGame::new();

        let i1 = ChessNotationPosition::new('e', 4).to_position();
        let i2 = ChessNotationPosition::new('f', 5).to_position();
        let i3 = ChessNotationPosition::new('g', 4).to_position();
        let i4 = ChessNotationPosition::new('a', 5).to_position();
        let i5_ambiguous = ChessNotationPosition::new('f', 5).to_position();

        let m1 = game.movement_from_chess_notation(i1, None, None, None, false);
        game.move_piece(m1.unwrap()).unwrap();
        let m2 = game.movement_from_chess_notation(i2, None, None, None, false);
        game.move_piece(m2.unwrap()).unwrap();
        let m3 = game.movement_from_chess_notation(i3, None, None, None, false);
        game.move_piece(m3.unwrap()).unwrap();
        let m4 = game.movement_from_chess_notation(i4, None, None, None, false);
        game.move_piece(m4.unwrap()).unwrap();
        let m5_ambiguous = game.movement_from_chess_notation(i5_ambiguous, None, None, None, true);

        assert_eq!(m5_ambiguous, Err(String::from("Ambiguous move!")));

        let m5_ambiguous = game.movement_from_chess_notation(i5_ambiguous, None,  Some(i1.rank), None, true);

        assert_eq!(m5_ambiguous, Err(String::from("Ambiguous move!")));

        let m5 = game.movement_from_chess_notation(i5_ambiguous, None, None, Some(i1.file), true);

        assert_eq!(m5, Ok(Movement::new(ChessNotationPosition::new('e', 4).to_position(), ChessNotationPosition::new('f', 5).to_position())));
    }
}