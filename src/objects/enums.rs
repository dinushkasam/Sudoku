use crate::objects::GameRules;

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

impl Difficulty {
    pub fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Extreme => "Extreme",
        }
    }

    pub fn all() -> Vec<Difficulty> {
        vec![
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::Hard,
            Difficulty::Extreme,
        ]
    }

    pub fn rules(&self) -> GameRules {
        match self {
            Difficulty::Easy => GameRules {
                max_mistakes: Some(10),
                mistake_highlight: true,
                max_undos: Some(10),
            },
            Difficulty::Medium => GameRules {
                max_mistakes: Some(7),
                mistake_highlight: true,
                max_undos: Some(7),
            },
            Difficulty::Hard => GameRules {
                max_mistakes: Some(5),
                mistake_highlight: true,
                max_undos: Some(5),
            },
            Difficulty::Extreme => GameRules {
                max_mistakes: Some(3),
                mistake_highlight: false,
                max_undos: Some(3),
            },
        }
    }
}

pub enum InputReturnType {
    Exit,
    CheckPuzzle,
    Continue,
}
