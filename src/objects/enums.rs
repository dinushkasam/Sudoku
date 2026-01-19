#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Given(u8),
    Filled(u8),
}

impl Cell {
    pub fn value(&self) -> Option<u8> {
        match self {
            Cell::Given(num) | Cell::Filled(num) => Some(*num),
            Cell::Empty => None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

pub enum InputReturnType {
    Exit,
    CheckPuzzle,
    Continue,
}
