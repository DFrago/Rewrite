use rewrite::{initial_setup,Player,Inventory,Item};
use crate::levels::*;
pub mod levels;


fn main(){
    let mut player:Player=initial_setup();
    let mut inventory=Inventory{player:&mut player,equipped:Item::empty(),items:Vec::new()};
    level_1::run(&mut inventory);
    level_2::run(&mut inventory);
    level_3::run(&mut inventory);
    level_4::run(&mut inventory);
    level_5::run(&mut inventory);
}
