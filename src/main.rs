use crate::{generator::generate_sudoku, objects::{Board, Cell::{Empty, Given}, Difficulty}, solver::{solve, solve_random}, validation_engine::is_valid};

mod generator;
mod objects;
mod solver;
mod validation_engine;

fn main() {
    // let mut board = Board::empty();
    // board.cells = [
    //     [Empty, Given(6), Given(7), Empty, Empty, Given(1), Empty, Empty, Empty],
    //     [Empty, Empty, Empty, Given(8), Given(5), Given(2), Empty, Empty, Empty],
    //     [Empty, Empty, Given(3), Empty, Empty, Given(7), Empty, Empty, Empty],
    //     [Empty, Empty, Empty, Empty, Given(8), Empty, Given(1), Empty, Empty],
    //     [Given(9), Empty, Given(4), Empty, Empty, Empty, Given(6), Empty, Empty],
    //     [Empty, Empty, Empty, Given(9), Empty, Empty, Empty, Given(7), Empty],
    //     [Empty, Given(5), Empty, Empty, Empty, Empty, Empty, Empty, Empty],
    //     [Given(7), Empty, Empty, Empty, Empty, Empty, Empty, Given(5), Given(2)],
    //     [Empty, Given(2), Empty, Given(1), Empty, Empty, Empty, Empty, Given(8)]
    // ];
    // solve_random(&mut board);

    let sudoku = generate_sudoku(Difficulty::Easy);
    sudoku.puzzle.print_board();
    sudoku.solution.print_board();
}
