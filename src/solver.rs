// Sudoku solver
// Solve a Sudoku grid using backtracking
// Constraint checking
use rand::{seq::SliceRandom, rng};
use crate::{objects::{Board, Cell}, validation_engine::is_valid};

fn find_empty(board: &Board) -> Option<(usize, usize)> {
    for r in 0..9 {
        for c in 0..9 {
            if board.cells[r][c] == Cell::Empty {
                return Some((r, c));
            }
        }
    }
    None
}

pub fn solve(board: &mut Board) -> bool {
    if let Some((r, c)) = find_empty(board) {
        for n in 1..=9 {
            if is_valid(board, r, c, n) {
                board.cells[r][c] = Cell::Filled(n);
                if solve(board) {
                    return true;
                }
                board.cells[r][c] = Cell::Empty;
            }
        }
        return false;
    }
    true
}

/// Used for Puzzle Generation
pub fn solve_random(board: &mut Board) -> bool {
    if let Some((r, c)) = find_empty(board) {
        let mut nums: Vec<u8> = (1..=9).collect();
        nums.shuffle(&mut rng());

        for num in nums {
            if is_valid(board, r, c, num) {
                board.cells[r][c] = Cell::Given(num);
                if solve_random(board) {
                    return true;
                }
                board.cells[r][c] = Cell::Empty;
            }
        }
        return false;
    }
    true
}

pub fn count_solutions(board: &mut Board, limit: usize) -> usize {
    if let Some((r, c)) = find_empty(board) {
        let mut solutions = 0;
        for n in 1..=9 {
            if is_valid(board, r, c, n) {
                board.cells[r][c] = Cell::Filled(n);
                solutions += count_solutions(board, limit);
                board.cells[r][c] = Cell::Empty;

                if solutions >= limit {
                    return solutions;
                }
            }
        }
        return solutions;
    }
    1
}