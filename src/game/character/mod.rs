use crate::game::character::character::{Player, Position, Character};

pub mod character;

pub struct PlayerBuilder<'a> {
    position: &'a Position
}


impl<'a> PlayerBuilder <'a> {
    pub fn new(position : &'a Position) -> PlayerBuilder {

        PlayerBuilder {
             position
        }
    }

    /// Executes the command as a child process, which is returned.
    pub fn spawn(&self) -> Player {
      Player::new( self.position )
    }
}