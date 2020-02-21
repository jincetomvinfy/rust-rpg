pub trait Character<'a> {
    fn new(position: & 'a Position) -> Self;
    fn set_position(&mut self, position: &'a Position);
    fn get_position(&self) -> &'a Position;
}

#[derive(Debug,PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Player<'a> {
    position:  &'a Position,
}

impl<'a> Character<'a> for Player<'a> {
    fn new(position: & 'a Position) -> Self {
         Player {position}
    }

    fn set_position(&mut self, position: & 'a Position) {
        self.position = position;
    }
    fn get_position(&self) -> &'a Position{
        self.position
    }

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn position_update() {
        let position=Position{
            x:10,
            y:10
        };
        let mut player=Player{
            position:&position
        };
        let new_position=Position{
            x:20,
            y:20
        };
        player.set_position(&new_position);
        assert_eq!(player.get_position(), &new_position);
    }
}