// Sudoku grid generator
// Remove numbers while keeping a unique solution

use rand::seq::SliceRandom;

use crate::{
    objects::{Board, Cell, Difficulty, GeneratedSudoku},
    solver::{count_solutions, solve, solve_random},
};

pub fn generate_sudoku(difficulty: Difficulty) -> GeneratedSudoku {
    let mut solution = Board::empty();
    solve_random(&mut solution);

    let mut puzzle = solution.clone();
    let mut num_to_remove = match difficulty {
        Difficulty::Easy => 30,
        Difficulty::Medium => 40,
        Difficulty::Hard => 50,
        Difficulty::Extreme => 60,
    };

    let mut positions = Vec::new();
    for r in 0..9 {
        for c in 0..9 {
            positions.push((r, c));
        }
    }
    positions.shuffle(&mut rand::rng());

    for (r, c) in positions.iter().take(num_to_remove) {
        let backup = puzzle.cells[*r][*c];
        puzzle.cells[*r][*c] = Cell::Empty;
        if !count_solutions(&mut puzzle, 2) == 1 {
            puzzle.cells[*r][*c] = backup;
            continue;
        }
        num_to_remove += 1;
    }

    GeneratedSudoku {
        puzzle,
        solution,
        difficulty,
    }
}
