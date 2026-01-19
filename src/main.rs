use crate::game_loop::run_game;

mod game_loop;
mod generator;
mod objects;
mod solver;
mod terminal;
mod validation_engine;

fn main() {
    match run_game() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }
}
