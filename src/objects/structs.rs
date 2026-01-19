use crate::{objects::Difficulty, terminal::restore_terminal};

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
}

pub struct Game {
    pub difficulty: Difficulty,
    pub board: Board,
    pub solution: Board,
    pub player: Player,
    pub state: GameState,
    pub rules: GameRules,
    pub mistakes: usize,
}

impl Game {
    pub fn new(difficulty: Difficulty, board: Board, solution: Board, player: Player) -> Self {
        Self {
            difficulty,
            board,
            solution,
            player,
            state: GameState::Playing,
            rules: difficulty.rules(),
            mistakes: 0,
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

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Solved,
    Failed,
}

pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        match restore_terminal() {
            Ok(()) => (),
            Err(e) => println!("Error restoring terminal: {}", e),
        }
    }
}

pub struct GameRules {
    pub max_mistakes: Option<usize>,
    pub mistake_highlight: bool,
}
