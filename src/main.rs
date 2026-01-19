use crate::{
    game_loop::{handle_cell_input, handle_movement, is_solved, read_input, run_game},
    generator::generate_sudoku,
    objects::{Difficulty, Game, GameState, Player},
    terminal::{render_board, setup_terminal},
};

mod game_loop;
mod generator;
mod objects;
mod solver;
mod terminal;
mod validation_engine;

fn main() {
    let sudoku = generate_sudoku(Difficulty::Easy);
    let player = Player::new();
    let mut game = Game::new(sudoku.puzzle, sudoku.solution, player);

    match run_game(&mut game) {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
}
