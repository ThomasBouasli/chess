use std::fmt::Display;

use super::absolute_position::AbsolutePosition;

#[derive(PartialEq, Debug)]
pub struct RelativePosition{
    pub file: i8,
    pub rank: i8,
}

impl RelativePosition{

    pub fn new(file: i8, rank: i8) -> Self {
        Self {
            file,
            rank,
        }
    }

    pub fn to_absolute(&self, position: &AbsolutePosition) -> Result<AbsolutePosition, String> {
        let file = self.file + position.file as i8;
        let rank = self.rank + position.rank as i8;

        if file < 0 || file > 7 || rank < 0 || rank > 7 {
            Err(format!("Invalid position: {}{}", file, rank))
        } else {
            Ok(AbsolutePosition {
                file: file as usize,
                rank: rank as usize,
            })
        }
    }

    pub fn from_absolute(from : &AbsolutePosition, to : &AbsolutePosition) -> RelativePosition {
        RelativePosition {
            file: to.file as i8 - from.file as i8,
            rank: to.rank as i8 - from.rank as i8,
        }
    }
}

impl Display for RelativePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

#[test]
fn test_position_to_absolute() {
    let position = RelativePosition::new(-1, -1);
    let absolute_position = match position.to_absolute(&AbsolutePosition::new(1, 1)) {
        Ok(position) => position,
        Err(_) => panic!("Invalid position!"),
    };


    assert_eq!(absolute_position.file, 0);
    assert_eq!(absolute_position.rank, 0);
}

#[test]
fn test_position_from_absolute() {
    let position = RelativePosition::new(-1, -1);
    let relative_position = RelativePosition::from_absolute(&AbsolutePosition::new(1, 1), &AbsolutePosition::new(0, 0));
    assert_eq!(relative_position, position);
}

#[test]
fn test_position_from_absolute_invalid() {
    let position = RelativePosition::new(-1, -1);
    let relative_position = RelativePosition::from_absolute(&AbsolutePosition::new(1, 1), &AbsolutePosition::new(1, 0));
    assert_ne!(relative_position, position);
}