use std::fmt::Error;
use crate::game::game::{Game};
use crate::game::character::character::{Position, Player};
use crate::game::GameBuilder;
use crate::game::context::GameContext;
use crate::game::character::PlayerBuilder;

mod game;
pub mod menu;
pub fn run() -> Result<(), Error> {
    let position=Position{x:1,y:2};
    let player_builder = PlayerBuilder::new(&position);
    let player:Player = player_builder.spawn();
    let context:GameContext = GameContext::new();
    let mut builder = GameBuilder::new(&player);
    let game:Game = builder.context(&context).spawn();
    game.start();
    Result::Ok(())
}