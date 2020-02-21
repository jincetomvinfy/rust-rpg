use rpg_lib::run;
fn main() {
match run() {
    Err(err)=>println!("Error occurred {}",err),
    _ => println!("Successfully run"),
}
}
