use crate::menu::display_main_menu;
use crate::game::context::GameContext;
use crate::game::character::character::Player;


pub struct Game<'a>{
    player:& 'a Player<'a>,
    context: Option<&'a GameContext>
}

impl<'a> Game<'a>{
    pub fn new (player: &'a Player,context:Option<&'a GameContext>) -> Game<'a>{
        Game{
            player,
            context
        }
    }
    pub fn start(&self){
       match   self.context{
           Some(context)=>println!("started at {}",context.started_at()),
           None => println!("started" )
       }

        display_main_menu();
    }

}
