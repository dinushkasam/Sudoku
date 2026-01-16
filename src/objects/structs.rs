use super::enums::Cell;

pub struct Board {
    pub cells: [[Cell; 9]; 9]
}

impl Board {
    pub fn empty() -> Self {
        Self { cells: [[Cell::Empty; 9]; 9] }
    }

    pub fn print_board() {
        // Impl
    }
}

pub struct Game {
    pub board: Board,
    pub player: Player,
    pub revealed: [[bool; 9]; 9],
}

pub struct Player {
    pub row: usize,
    pub col: usize,
}

pub type Location = [u8; 2];
