use crate::objects::Difficulty;

use super::enums::Cell;

#[derive(Clone)]
pub struct Board {
    pub cells: [[Cell; 9]; 9],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            cells: [[Cell::Empty; 9]; 9],
        }
    }

    pub fn print_board(&self) {
        println!("+-------+-------+-------+");
        for (i, row) in self.cells.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                println!("+-------+-------+-------+");
            }
            print!("| ");
            for (j, cell) in row.iter().enumerate() {
                if j > 0 && j % 3 == 0 {
                    print!("| ");
                }
                match cell {
                    Cell::Empty => print!(". "),
                    Cell::Given(n) | Cell::Filled(n) => print!("{} ", n),
                }
            }
            println!("|");
        }
        println!("+-------+-------+-------+");
    }
}

pub struct Game {
    pub board: Board,
    pub solution: Board,
    pub player: Player,
}

impl Game {
    pub fn new(board: Board, solution: Board, player: Player) -> Self {
        Self {
            board,
            solution,
            player,
        }
    }
}

pub struct Player {
    pub row: usize,
    pub col: usize,
}

impl Player {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }
}

pub struct GeneratedSudoku {
    pub puzzle: Board,
    pub solution: Board,
    pub difficulty: Difficulty,
}
