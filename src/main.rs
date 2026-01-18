use crate::{
    game_loop::{handle_cell_input, handle_movement, is_solved, read_input, render_board},
    generator::generate_sudoku,
    objects::{
        Board,
        Cell::{Empty, Given},
        Difficulty, Game, Player,
    },
    solver::{solve, solve_random},
    validation_engine::is_valid,
};

mod game_loop;
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
    let mut player = Player::new();
    let mut game = Game::new(sudoku.puzzle, sudoku.solution, player);

    loop {
        render_board(&game.board, &game.player);

        let input = match read_input() {
            Ok(input) => input,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        if input == "q" {
            break;
        }

        handle_movement(&input, &mut game.player);

        match handle_cell_input(&input, &mut game) {
            Ok(()) => {
                if is_solved(&game.board, &game.solution) {
                    println!("You solved the puzzle!");
                    break;
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
