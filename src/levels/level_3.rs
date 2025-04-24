use rewrite::*;
use rewrite::Speed;

pub fn max(text:&str){animate(&format!("Max: {}",text),Speed::DEFAULT,&Sound::NAME("max.mp3"));}

pub fn run(inventory:&mut Inventory){
    intro(inventory);
}

fn intro(inventory:&mut Inventory){
    double_line();
    display_file("maxs_court_ascii",Speed::HEADER,&Sound::NAME("maxs_court.mp3"));
    display_file("max",Speed::IMAGE,&Sound::NONE);
    text("You arrive at the new wellness center and see that Max is already there");
    max(&format!("{}! Whats up dude?",inventory.player.get_name()));
    max(&format!("I've got a cooler full of margs. Have one bro"));
    inventory.add(Item{item_type:ItemType::CONSUMABLE,value:50,name:String::from("Marg")});
    inventory.player.speak("What are you up to?");
    max("Just having a sick pancho sesh");
    inventory.player.speak("That sounds nice");
    inventory.player.speak("I could use a pancho sesh since I've been rewritting everyones programs in Rust");
    max("Why bro?");
    max("Rust is still immature");
    max("Nobody is building million dollar companies with Rust rn");
    play_single_sound(&Sound::NAME("capitalism.mp3"));
    inventory.player.speak("How dare you");
    max("The reality is if it aint making money it can sniff");
    let choices=vec![
        String::from("1) It's not about the money."),
        String::from("2) Rust is technically superior to all other langagues."),
        String::from("3) I didn't realize that you had gone full business main"),
        String::from("4) Rust will lead to a better bottom line")
    ];
    match make_choice(DMatrix{choices,correct:String::from("4")}){true=>handle_max_success(),false=>handle_max_failure(inventory)}
}

pub fn handle_max_success(){
    play_single_sound(&Sound::NAME("sheesh.mp3"));
    max("Good point bro.");
    max("Alright, enough technical talk");
    max("Lets play some volleyball!");
    text("After playing a few rounds, you dap up Max and head out");
    complete_max();
}
pub fn handle_max_failure(inventory:&mut Inventory){
    play_single_sound(&Sound::NAME("hell_nah.mp3"));
    max("Nah bro. You shouldn't be in STEM.");
    display_file("max",Speed::IMAGE,&Sound::NONE);

    combat(inventory,Enemy(Entity{name:String::from("Max"),health:20,strength:20,defense:5}),Sound::NAME("max_combat.mp3"));

    text("You watch as your slain enemy falls to the floor.");
    text("You wish it didn't have to be this way.");
    text("You look over at the chair Max had set up, and notice a book");
    inventory.add(Item{item_type:ItemType::EQUIPABLE,value:30,name:String::from("The Rust Book")});
    inventory.player.speak("Huh, maybe Max really was into rust afterall...");
    complete_max();
}
pub fn complete_max(){
    complete_level("Maxs Court",&Sound::NAME("level_complete.mp3"));
    text("You think about how much convincing it's taken to get people to see the truth of Rust");
    text("You figure if anyone is unconditionally on the side of Rust, it would be Dr. Sat");
    text("You head to his office");
}

