use crate::game::game::{ Game};
use crate::game::context::GameContext;
use crate::game::character::character::Player;

pub mod game;
pub mod context;
pub mod character;

pub struct GameBuilder<'a> {
    player: &'a Player<'a>,
    context : Option<&'a GameContext>
}


impl<'a> GameBuilder<'a> {
    pub fn new(player: &'a Player<'a>) -> GameBuilder {
        GameBuilder {
            player,
            context: None
        }
    }

    /// Add an argument to pass to the program.
    pub fn context(&'a mut self, context: &'a GameContext) -> &'a mut GameBuilder {
        self.context=Some(context);
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn spawn(&self) -> Game {
        Game::new(self.player,self.context)
    }
}