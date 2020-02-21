use crate::menu::menu::Menu;
use crate::menu::menu::MenuItem;
pub mod menu;
pub fn display_main_menu(){
    let items = vec![MenuItem{ name: "PLAY", position: 1 }, MenuItem{ name: "EXIT", position: 2 }];
    let main_menu=Menu{ items: &items };
    display_menu(&main_menu)
}
fn display_menu(menu:&Menu){
    for item in menu.items{
        println!("{}.{}",item.position,item.name)
    }

}