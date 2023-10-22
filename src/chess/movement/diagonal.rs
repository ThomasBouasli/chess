use super::relative_position::RelativePosition;


pub trait DiagonalMovement {
    fn diagonal_movement(&self, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if relative_position.file.abs() != 0 && relative_position.file.abs() == relative_position.rank.abs() {
            let mut movement_path = Vec::new();
            for n in 1..relative_position.file.abs() {
                movement_path.push(RelativePosition {
                    file: n * relative_position.file.signum(),
                    rank: n * relative_position.rank.signum(),
                });
            }

            (movement_path, true)
        } else {
            (Vec::new(), false)
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    struct TestStruct;

    impl DiagonalMovement for TestStruct{}

    #[test]
    fn test_can_move_vertically_one_tile(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 1, rank: 1};
        let (movement_path, can_move) = test_struct.diagonal_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_in_the_opposite_direction(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: -1, rank: -1};
        let (movement_path, can_move) = test_struct.diagonal_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_any_number_of_tiles(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 5, rank: -5};
        let (movement_path, can_move) = test_struct.diagonal_movement(&relative_position);

        let mut expected_vector = Vec::new();

        
        for n in 1..5{
            expected_vector.push(RelativePosition{file: n, rank: -n});
        }

        assert_eq!(movement_path, expected_vector);
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_cannot_move_in_line(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 0, rank: 1};
        let (movement_path, can_move) = test_struct.diagonal_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, false);
    }
}