use rewrite::*;
use rewrite::Speed;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

pub fn risc(text:&str){animate(&format!("Risc Wrecker: {}",text),Speed::DEFAULT,&Sound::NAME("risc_wreaker.wav"));}
pub fn sat(text:&str){animate(&format!("Dr.Sat: {}",text),Speed::DEFAULT,&Sound::NAME("sat.wav"));}

pub fn run(inventory:&mut Inventory){
    intro(inventory);
}

fn intro(inventory:&mut Inventory){
    double_line();
    risc("You fool");
    thread::sleep(Duration::from_millis(1000));
    risc("You have activated my trap card");
    display_file("risc",Speed::IMAGE,&Sound::NONE);
    combat(inventory,Enemy(Entity{name:String::from("Risc Wrecker"),health:70,strength:50,defense:15}),Sound::NAME("risc_combat.mp3"));
    
    let(tx,rx)=mpsc::channel(); 
    let handle=Some(play_sound(rx,Sound::NAME("sat_death.mp3")));
    text("You watch as the Risc Wrecker slowly returns to his Dr.Sat form");
    text("Dr.Sat looks like he is on the edge of death");
    thread::sleep(Duration::from_millis(1000));
    sat("I would have helped you..");
    thread::sleep(Duration::from_millis(1000));
    sat("Make Rust the future...");
    thread::sleep(Duration::from_millis(2000));
    sat("If you could just understand...");
    thread::sleep(Duration::from_millis(3000));
    sat("That it is one of many tools");
    thread::sleep(Duration::from_millis(1000));
    sat(".......");
    thread::sleep(Duration::from_millis(3000));
    sat("You are blinded by your passion....");
    thread::sleep(Duration::from_millis(2000));
    sat("My student...");
    thread::sleep(Duration::from_millis(3000));
    sat("If my death has taught you something...");
    thread::sleep(Duration::from_millis(2000));
    sat("Then perhaps....");
    thread::sleep(Duration::from_millis(3000));
    sat("I've done my job as a teacher");
    play_single_sound(&Sound::NAME("enemy_death.mp3"));
    text("Dr. Sat has died");
    thread::sleep(Duration::from_millis(3000));
    inventory.player.speak("Maybe Dr.Sat was right");
    thread::sleep(Duration::from_millis(1000));
    inventory.player.speak("Maybe the programming world is big");
    thread::sleep(Duration::from_millis(1000));
    inventory.player.speak("....");
    thread::sleep(Duration::from_millis(3000));
    inventory.player.speak("And maybe Rust is only one part of it");
    thread::sleep(Duration::from_millis(3000));
    text("Anyways..");
    text("THANKS FOR PLAYING!");
    let _=tx.send(());
    if let Some(h)=handle{h.join().unwrap();}
}
