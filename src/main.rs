use bracket_terminal::prelude::*;
use game::Control;

mod game;

fn main() -> BError {
    let gameState = game::Control::new();
    let bTermContext = BTermBuilder::simple80x50()
        .with_title("Rusty Snake")
        .build()?;

    main_loop( bTermContext, gameState)
}