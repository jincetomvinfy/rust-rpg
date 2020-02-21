use std::vec::Vec;
pub struct Menu<'a>{
    pub items:&'a Vec<MenuItem>,
}
pub struct  MenuItem{
    pub name:&'static str,
    pub position:i32

}



