use std::error::Error;
use crate::engine::game::Game;

mod engine;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let game = Game::new_random(2, 25)?;

    println!("{:?}", game);

    Ok(())
}
