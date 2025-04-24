use rewrite::*;
use rewrite::Speed;
use std::thread;
use std::time::Duration;

pub fn sat(text:&str){animate(&format!("Dr.Sat: {}",text),Speed::DEFAULT,&Sound::NAME("sat.wav"));}

pub fn run(inventory:&mut Inventory){
    intro(inventory);
}

fn intro(inventory:&mut Inventory){
    double_line();
    display_file("sats_office_ascii",Speed::HEADER,&Sound::NAME("sats_office.mp3"));
    display_file("sat",Speed::IMAGE,&Sound::NONE);
    text("You arrive at Dr.Sats office");
    sat(&format!("Hello {}",inventory.player.get_name()));
    sat("What are you here for today?");
    inventory.player.speak("Well Dr.Sat, I wanted to talk to you about Rust");
    inventory.player.speak("I figured if anybody would understand, it would be you");
    inventory.player.speak("Please tell me that you agree that Rust is the one true langauge");
    sat("Well Rust is a great langauge with a promising future");
    sat("But it is one tool for the toolbelt");
    sat("You should use Rust where it's strengths shine, but don't try to make every problem a Rust problem");
    inventory.player.speak("Et tu Brute?");
    inventory.player.speak("Dr.Sat. Rust is the one true langauge");
    sat("It would be unwise to think that way");
    let choices=vec![
        String::from("1) I used to look up to you."),
        String::from("2) I'm going to write you a bad course evaluation."),
        String::from("3) You were supposed to destroy the pythonistas, not join them."),
        String::from("4) Are you the strongest because you are chad, or are you chad because you are the strongest?")
    ];
    match make_choice(DMatrix{choices,correct:String::from("1")}){true=>handle_sat_failure(inventory),false=>handle_sat_failure(inventory)}
}


pub fn handle_sat_failure(inventory:&mut Inventory){
    play_single_sound(&Sound::NAME("sat_fail.mp3"));
    sat("Ever heard of the Sat special?");
    display_file("sat",Speed::IMAGE,&Sound::NONE);

    combat(inventory,Enemy(Entity{name:String::from("Dr.Sat"),health:50,strength:20,defense:1}),Sound::NAME("sat_combat.mp3"));

    text("You watch as your slain enemy falls to the floor.");
    complete_sat();
}
pub fn complete_sat(){
    complete_level("Dr.Sats Office",&Sound::NAME("level_complete.mp3"));
    thread::sleep(Duration::from_millis(2000));
    animate(".....",Speed::DIED,&Sound::NONE);
    animate("..........",Speed::DIED,&Sound::NONE);
    animate("...............",Speed::DIED,&Sound::NONE);
}


