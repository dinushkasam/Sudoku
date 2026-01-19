use crate::{
    game_loop::run_game,
    generator::generate_sudoku,
    objects::{Difficulty, Game, Player},
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
    let mut game = Game::new(sudoku.difficulty, sudoku.puzzle, sudoku.solution, player);

    match run_game(&mut game) {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
}
