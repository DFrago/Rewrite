use rewrite::{initial_setup,Player,Inventory,Item};
use crate::levels::*;
pub mod levels;


fn main(){
    let mut player:Player=initial_setup();
    //let mut player=Player(Entity{name:String::from("Chungus"),health:50,defense:10,strength:15});
    let mut inventory=Inventory{player:&mut player,equipped:Item::empty(),items:Vec::new()};
    //inventory.add(Item{item_type:ItemType::CONSUMABLE,value:5,name:String::from("Coffee")});
    //inventory.add(Item{item_type:ItemType::EQUIPABLE,value:2,name:String::from("Copilot")});
    //inventory.add(Item{item_type:ItemType::EQUIPABLE,value:5,name:String::from("Cursor")});
    //inventory.add(Item{item_type:ItemType::CONSUMABLE,value:20,name:String::from("Yerba")});
    //inventory.add(Item{item_type:ItemType::CONSUMABLE,value:30,name:String::from("Marg")});
    //inventory.add(Item{item_type:ItemType::EQUIPABLE,value:30,name:String::from("The Rust Book")});
    level_1::run(&mut inventory);
    level_2::run(&mut inventory);
    level_3::run(&mut inventory);
    level_4::run(&mut inventory);
    level_5::run(&mut inventory);
}
