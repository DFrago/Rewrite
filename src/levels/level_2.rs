use rewrite::*;
use rewrite::Speed;

pub fn cory(text:&str){animate(&format!("Cory: {}",text),Speed::DEFAULT,&Sound::NAME("cory.mp3"));}

pub fn run(inventory:&mut Inventory){
    intro(inventory);
}

fn intro(inventory:&mut Inventory){
    double_line();
    display_file("corys_room_ascii",Speed::HEADER,&Sound::NAME("corys_room.mp3"));
    display_file("cory",Speed::IMAGE,&Sound::NONE);
    text("You enter into Corys study room and take a seat");
    cory(&format!("Hey, what's up {}",inventory.player.get_name()));
    inventory.player.speak("Not much bro, I was hoping we could study for the networking midterm coming up");
    cory("I would be down, but I got to finish this Neural Nets assignment");
    cory("Actually, now that you are here, would you mind taking a look at it?");
    inventory.player.speak("Sure bro");
    text("You look over at Corys Macbook Pro");
    inventory.player.speak("...");
    inventory.player.speak("......");
    inventory.player.speak(".........");
    inventory.player.speak("Cory, what is this?");
    cory("What do you mean?");
    inventory.player.speak("I still would have raged, but I would have been a little bit more understanding if this was written in Python");
    inventory.player.speak("BUT THIS IS WRITTEN IN JAVA!");
    play_single_sound(&Sound::NAME("boom.mp3"));
    cory("Well, Java is a learning objective...");
    inventory.player.speak("Brother please....let me rewrite your assignment in rust");
    cory("That is an option I could consider...");

    let choices=vec![
        String::from("1) This code smells like something a cubs fan would write..."),
        String::from("2) Rust might be on the midterm."),
        String::from("3) Rust is a valuable skill to have"),
        String::from("4) If you want to learn deeply, you need to first learn Rust.")
    ];
    match make_choice(DMatrix{choices,correct:String::from("2")}){true=>handle_cory_success(),false=>handle_cory_failure(inventory)}
}

pub fn handle_cory_success(){
    play_single_sound(&Sound::NAME("sheesh.mp3"));
    cory("True");
    cory("I didn't consider that Rust is a learning objective.");
    cory("Go ahead bro, rewrite my program in rust.");
    complete_cory();
}
pub fn handle_cory_failure(inventory:&mut Inventory){
    play_single_sound(&Sound::NAME("hell_nah.mp3"));
    cory("That's not a learning objective....");
    display_file("cory",Speed::IMAGE,&Sound::NONE);

    combat(inventory,Enemy(Entity{name:String::from("Cory"),health:20,strength:20,defense:5}),Sound::NAME("cory_combat.mp3"));

    text("You watch as your slain enemy falls to the floor.");
    text("If only he had understood that Rust is the one true langauge.");
    text("You notice that Cory dropped his Cursor.");
    inventory.add(Item{item_type:ItemType::EQUIPABLE,value:20,name:String::from("Cursor")});
    text("You also look through Corys backpack, and find a yerba mate");
    inventory.add(Item{item_type:ItemType::CONSUMABLE,value:30,name:String::from("Yerba")});

    complete_cory();
}
pub fn complete_cory(){
    text("You proceed to do the lord's work, and rewrite Cory's program in rust.");
    complete_level("Corys Room",&Sound::NAME("level_complete.mp3"));
    text("You decide that you've been working pretty hard and could use a break");
    text("You decide to go to the new volleyball courts to blow off some steam");
}

