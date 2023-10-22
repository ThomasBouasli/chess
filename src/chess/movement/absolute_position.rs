use std::fmt::Display;

use super::relative_position::RelativePosition;


#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub struct  AbsolutePosition{
    pub file: usize,
    pub rank: usize,
}


impl Display for AbsolutePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", (self.file as u8 + 97) as char, self.rank + 1)
    }
}

impl AbsolutePosition {

    pub fn new(file: usize, rank: usize) -> Self {
        Self {
            file,
            rank,
        }
    }

    pub fn to_relative(&self, position: &AbsolutePosition) -> RelativePosition {
        RelativePosition {
            file: (self.file as i8 - position.file as i8),
            rank: (self.rank as i8 - position.rank as i8),
        }
    }

    pub fn from_relative(from : &AbsolutePosition, to : &RelativePosition) -> Result<AbsolutePosition, String> {
        let file = to.file + from.file as i8;
        let rank = to.rank + from.rank as i8;

        if file < 0 || file > 7 || rank < 0 || rank > 7 {
            Err(format!("Invalid position!"))
        } else {
            Ok(AbsolutePosition {
                file: file as usize,
                rank: rank as usize,
            })
        }
    }
}


#[test]
fn test_position_to_relative() {
    let position = AbsolutePosition::new(0, 0);
    let relative_position = position.to_relative(&AbsolutePosition::new(1, 1));
    assert_eq!(relative_position.file, -1);
    assert_eq!(relative_position.rank, -1);
}

#[test]
fn test_position_from_relative() {
    let position = AbsolutePosition::new(0, 0);
    let relative_position = RelativePosition::new(1, 1);
    

    let position = AbsolutePosition::from_relative(&position, &relative_position).unwrap();

    assert_eq!(position.file, 1);
    assert_eq!(position.rank, 1);
}

#[test]
fn test_should_fail_position_from_relative() {
    let position = AbsolutePosition::new(0, 0);
    let relative_position = RelativePosition::new(-1, 0);
    

    let position = AbsolutePosition::from_relative(&position, &relative_position);

    assert!(position.is_err());
}