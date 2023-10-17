use super::RelativePosition;

pub trait LineMovement {
    fn horizontal_movement(&self, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if relative_position.file.abs() == 0 && relative_position.rank.abs() != 0 {
            let mut movement_path = Vec::new();
            for n in 1..relative_position.rank.abs() {
                movement_path.push(RelativePosition {
                    file: 0,
                    rank: n * relative_position.rank.signum(),
                });
            }

            (movement_path, true)
        } else {
            (Vec::new(), false)
        }
    }

    fn vertical_movement(&self, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        if relative_position.file.abs() != 0 && relative_position.rank.abs() == 0 {
            let mut movement_path = Vec::new();
            for n in 1..relative_position.file.abs() {
                movement_path.push(RelativePosition {
                    file: n * relative_position.file.signum(),
                    rank: 0,
                });
            }

            (movement_path, true)
        } else {
            (Vec::new(), false)
        }
    }

    fn line_movement(&self, relative_position: &RelativePosition) -> (Vec<RelativePosition>, bool) {
        let (horizontal_movement_path, horizontal_can_move) = self.horizontal_movement(relative_position);
        let (vertical_movement_path, vertical_can_move) = self.vertical_movement(relative_position);

        let mut movement_path = Vec::new();
        movement_path.extend(horizontal_movement_path);
        movement_path.extend(vertical_movement_path);

        (movement_path, horizontal_can_move || vertical_can_move)
    }
}

#[cfg(test)]
mod tests{
    use crate::chess::movement::RelativePosition;

    use super::*;

    struct TestStruct;

    impl LineMovement for TestStruct{}

    #[test]
    fn test_can_move_horizontally_one_square(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 0, rank: 1};
        let (movement_path, can_move) = test_struct.horizontal_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_in_the_opposite_direction(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 0, rank: -1};
        let (movement_path, can_move) = test_struct.horizontal_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_any_number_of_squares(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 0, rank: 5};
        let (movement_path, can_move) = test_struct.horizontal_movement(&relative_position);

        let mut expected_vector = Vec::new();

        for n in 1..5{
            expected_vector.push(RelativePosition{file: 0, rank: n});
        }

        assert_eq!(movement_path, expected_vector);
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_vertically_one_square(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 1, rank: 0};
        let (movement_path, can_move) = test_struct.vertical_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_in_the_opposite_direction_vertically(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: -1, rank: 0};
        let (movement_path, can_move) = test_struct.vertical_movement(&relative_position);
        assert_eq!(movement_path, Vec::new());
        assert_eq!(can_move, true);
    }

    #[test]
    fn test_can_move_any_number_of_squares_vertically(){
        let test_struct = TestStruct;
        let relative_position = RelativePosition{file: 5, rank: 0};
        let (movement_path, can_move) = test_struct.vertical_movement(&relative_position);

        let mut expected_vector = Vec::new();

        for n in 1..5{
            expected_vector.push(RelativePosition{file: n, rank: 0});
        }

        assert_eq!(movement_path, expected_vector);
        assert_eq!(can_move, true);
    }
}