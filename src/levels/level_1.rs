use rewrite::*;
use rewrite::Speed;

pub fn elbron(text:&str){animate(&format!("Elbron: {}",text),Speed::DEFAULT,&Sound::NAME("sans.mp3"));}

pub fn run(inventory:&mut Inventory){
    intro(inventory);
}

fn intro(inventory:&mut Inventory){
    double_line();
    display_file("elbrons_lab_ascii",Speed::HEADER,&Sound::NAME("elbrons_lab.mp3"));
    text("You slowly open your eyes, feeling the effects of the copious amounts of alcohol you drank the night prior");
    text("You vaguely remember something about getting clapped in pickleball by Anthony"); 
    text("It's all coming back now"); 
    text("That was the motivation for said heavy drinking");
    text("As you awaken, you recall that shortly after finishing the rust book, you decided that you would dedicate your life to rewriting the world in rust"); 
    text("You vowed to guide the software world into a brighter future");
    text("No matter who stands in your way");
    text("Safe, fast, productive");
    text("Pick three");
    text("As you stand to your feet, you hear a familiar voice");
    display_file("elbron",Speed::IMAGE,&Sound::NONE);

    elbron(&format!("Hey {}, looks like you had a rough night last night.",inventory.player.get_name()));
    elbron("Here bro, have this Coffee, it'll perk you right up.");
    inventory.add(Item{item_type:ItemType::CONSUMABLE,value:10,name:String::from("Coffee")});
    elbron("Come check out this program I just wrote!");
    text("You take a look at his program.");
    inventory.player.speak("...");
    inventory.player.speak("......");
    inventory.player.speak(".........");
    inventory.player.speak("My brother in christ. This is written in Java.");
    inventory.player.speak("You have commited a grave sin.");
    inventory.player.speak("Allow me to rewrite your program in Rust.");
    elbron("I don't know bro...Java aint all that bad");

    let choices=vec![
        String::from("1) Bro,Rust is faster than java."),
        String::from("2) Professor Grogan said Rust is chad."),
        String::from("3) 49 20 74 6F 6F 20 61 6D 20 66 72 6F 6D 20 73 70 61 63 65."),
        String::from("4) Stinky A.I mains...you are all the same.")
    ];
    match make_choice(DMatrix{choices,correct:String::from("3")}){true=>handle_elbron_success(),false=>handle_elbron_failure(inventory)}
}

pub fn handle_elbron_success(){
    play_single_sound(&Sound::NAME("sheesh.mp3"));
    elbron("I didn't know you were chill like that.");
    elbron("Go ahead bro, rewrite my program in rust.");
    text("You proceed to do the lord's work, and rewrite Elbron's program in rust.");
    complete_elbron();
}
pub fn handle_elbron_failure(inventory:&mut Inventory){
    play_single_sound(&Sound::NAME("hell_nah.mp3"));
    elbron("NAH. Not good enough. Run them hands!");
    text("TIP: You are about to enter combat.");
    text("TIP: Enter attack to attack with the currently equipped item");
    text("TIP: Enter 'inventory' at any time to see what items you have.");
    text("TIP: Enter 'use' where you will prompted for an item currently in your inventory.");
    text("TIP: Use will equip any equippable item, or consume any consumable item.");
    text("TIP: Equippable items will increase you attack power, consumable items will restore your health.");
    text("TIP: Keep an eye on your health. If it goes to zero, you are cooked.");
    text("TIP: The stinky programmer who made this stinky game has not implemented checkpoints, so GG");
    display_file("elbron",Speed::IMAGE,&Sound::NONE);
    combat(inventory,Enemy(Entity{name:String::from("Elbron"),health:10,strength:15,defense:5}),Sound::NAME("elbron_combat.mp3"));
    text("You watch as your slain enemy falls to the floor.");
    text("He was a good friend, but he just couldn't see the truth that is rust.");
    text("You notice that Elbron dropped his Copilot.");
    inventory.add(Item{item_type:ItemType::EQUIPABLE,value:10,name:String::from("Copilot")});
    text("You then proceed to do the lord's work, and rewrite Elbron's program in rust.");
    complete_elbron();
}
pub fn complete_elbron(){
    complete_level("Elbrons Lab",&Sound::NAME("level_complete.mp3"));
    text("You decide that you should head to the library to study.");
    text("Cory said he had a study room reserved, so you go to meet him there.");
}
