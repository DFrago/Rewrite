use std::{io::{self,Write,BufReader},process,fs,thread};
use std::time::Duration;
use std::sync::mpsc;
use rand::Rng;
use rodio::*;

pub struct Entity{
    pub name:String,
    pub health:i32,
    pub strength:u32,
    pub defense:u32
}
pub struct Player(pub Entity);
pub struct Enemy(pub Entity);

#[derive(Clone)]
pub enum ItemType{CONSUMABLE,EQUIPABLE}

#[derive(Clone)]
pub struct Item{
    pub item_type:ItemType,
    pub name:String,
    pub value:i32
}
pub struct Inventory<'a>{
    pub player:&'a mut Player,
    pub equipped:Item,
    pub items:Vec<Item>
}

impl Item{
    pub fn empty()->Item{Item{item_type:ItemType::EQUIPABLE,name:String::from("None"),value:0}} 
    pub fn get_attribute(&self)->String{
        match self.item_type{
            ItemType::EQUIPABLE=>String::from(&format!("+{} Strength",self.value)),
            ItemType::CONSUMABLE=>String::from(&format!("+{} Health",self.value))
        }
    }
}

impl Inventory<'_>{
    pub fn display(&self){
        text("\nInventory");
        text(&format!("Equipped: {} ",self.equipped.name));
        text("Items:");

        match self.items.is_empty(){
            true=>text(" None "),
            false=>self.items.iter().for_each(|x|text(&format!(" {}:{} ",x.name,x.get_attribute())))
        }
    }

    pub fn add(&mut self,item:Item){
        animate(&format!("{} added to inventory!",item.name),Speed::DEFAULT,&Sound::NAME("got_item.mp3"));
        self.items.push(item);
    }
}

pub struct DMatrix{
    pub choices:Vec<String>,
    pub correct:String
}

pub trait HasEntity{
    fn entity(&self)->&Entity;
    fn entity_mut(&mut self)->&mut Entity;
    fn damage(&mut self,amount:i32){
        self.entity_mut().health-=amount;
        text(&format!("{} took {} damage",self.entity().name,amount));
    }

    fn show_stats(&self){
        let ent=self.entity();
        text(&format!("{}: Health:{} Strength:{} Defense:{}",ent.name,ent.health,ent.strength,ent.defense));
    }
    fn get_health(&self)->i32{self.entity().health}
    fn get_name(&self)->&String{&self.entity().name}
    fn get_strength(&self)->&u32{&self.entity().strength}
    fn get_defense(&self)->&u32{&self.entity().defense}
}

impl Player{
    fn create()->Self{
        let mut name=String::new();
        let max_chars=20;

        loop{
            text("Enter your player's name:");

            name.clear();
            io::stdin().read_line(&mut name).unwrap();
            let name=String::from(name.trim());
            let name_length=name.len();

            if name_length==0{text("Name cannot be empty");continue;}
            else if name_length>max_chars{text("20 character limit");continue;}
            check_for_quit(&name);        

            let mut verification=String::new();
            loop{
                verification.clear();
                text(&format!("You've entered {name}, is that correct? [y/n]"));
                io::stdin().read_line(&mut verification).unwrap();
                check_for_quit(&verification);
            
                match verification.trim().to_lowercase().as_str(){
                    "y"|"yes"=>return Self(Entity{name,health:60,defense:10,strength:15}),
                    "n"|"no"=>break,
                    _=>{
                        text("Invalid Response!");
                        continue;
                    }
                }
            }
        }
    }
    pub fn speak(&self,text:&str){
        animate(&format!("{}: {}",self.get_name(),text),Speed::DEFAULT,&Sound::NAME("player.mp3"));
    }
    pub fn heal(&mut self,amount:i32){self.entity_mut().health+=amount;}
}

impl HasEntity for Player{
    fn entity(&self)->&Entity{&self.0}
    fn entity_mut(&mut self)->&mut Entity{&mut self.0}
}
impl HasEntity for Enemy{
    fn entity(&self)->&Entity{&self.0}
    fn entity_mut(&mut self)->&mut Entity{&mut self.0}
}

fn check_for_quit(input:&str){if input.to_lowercase()=="quit"{process::exit(0)}}
pub fn double_line(){println!("\n")}

pub fn initial_setup()->Player{
    animate("Welcome to Rewrite!",Speed::MENU,&Sound::NONE);
    animate("Enter quit to exit the game.",Speed::MENU,&Sound::NONE);
    Player::create() 
}

pub enum Speed{DEFAULT,HEADER,MENU,IMAGE,COMPLETE,DIED,}
pub enum Sound{NONE,NAME(&'static str)}

pub fn animate(content:&str,speed:Speed,sound:&Sound){
    let(tx,rx)=mpsc::channel(); 

    let handle=match sound{
        Sound::NAME(name)=>Some(play_sound(rx,Sound::NAME(name))),
        Sound::NONE=>Some(play_sound(rx,Sound::NAME("text.wav"))),
    };
    

    let time=match speed{Speed::DEFAULT=>45,Speed::HEADER=>10,Speed::MENU=>40,Speed::IMAGE=>2,Speed::COMPLETE=>65,Speed::DIED=>200};

    print!("\n");
    for char in content.chars(){
        thread::sleep(Duration::from_millis(time));
        print!("{char}");
        io::stdout().flush().unwrap();
    }
    print!("\n");

    let _=tx.send(());
    if let Some(h)=handle{h.join().unwrap();}
}

pub fn play_single_sound(sound:&Sound){
    let mut _stream_and_sink=match sound{
        Sound::NAME(name)=>{
            let(_stream,_stream_handle)=OutputStream::try_default().unwrap();
            let _sink=Sink::try_new(&_stream_handle).unwrap();

            let file=fs::File::open(format!("resources/{}",name)).unwrap();
            let source=Decoder::new(BufReader::new(file)).unwrap();
            _sink.append(source);
            _sink.sleep_until_end();
            (Some(_stream),Some(_sink))
        }
        Sound::NONE=>(None,None)
        };
}

pub fn play_sound(stop_rx:mpsc::Receiver<()>,sound:Sound)->thread::JoinHandle<()>{
    thread::spawn(move||{
        let mut _stream_and_sink=match sound{
        Sound::NAME(name)=>{
            let(_stream,_stream_handle)=OutputStream::try_default().unwrap();
            let _sink=Sink::try_new(&_stream_handle).unwrap();

            let file=fs::File::open(format!("resources/{}",name)).unwrap();
            let source=Decoder::new(BufReader::new(file)).unwrap().repeat_infinite();
            _sink.append(source);
            (Some(_stream),Some(_sink))
        }
        Sound::NONE=>(None,None)
        };
        loop{
            match stop_rx.try_recv(){
                Ok(_)|Err(mpsc::TryRecvError::Disconnected)=>{
                    _stream_and_sink.1=None;
                    break;
                }
                Err(mpsc::TryRecvError::Empty)=>{
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    })
}

pub fn display_file(name:&str,speed:Speed,sound:&Sound){
    animate(&fs::read_to_string(format!("resources/{}",&name)).expect(&format!("Tried to read {}",&name)),
        speed,sound);
}

pub fn text(text:&str){animate(text,Speed::DEFAULT,&Sound::NONE);}

pub fn make_choice(matrix:DMatrix)->bool{
    matrix.choices.iter().for_each(|x|text(&format!("{x}")));

    let mut choice=String::new();
        loop{
            choice.clear();
            text("Choose Wisely");
            io::stdin().read_line(&mut choice).unwrap();
            check_for_quit(&choice);
        
            match choice.trim(){
                s if s==matrix.correct=>return true,
                "1"|"2"|"3"|"4"=>return false,
                _=>{
                    println!("Invalid Choice");
                    continue;
                }
            }
        }
}

pub fn complete_level(level:&str,sound:&Sound){
    animate(&format!("Congratulations. You have completed {level}!"),Speed::COMPLETE,sound);
}

pub fn you_died(){
    text("YOU DIED.");
    play_single_sound(&Sound::NAME("you_died.mp3"));
    check_for_quit("quit");
}

pub fn combat(inventory:&mut Inventory,mut enemy:Enemy,sound:Sound){
    let mut rng=rand::thread_rng();
    enum Reason{DEATH,VICTORY,INIT}
    let mut reason=Reason::INIT;
    let(tx,rx)=mpsc::channel(); 
    let handle=Some(play_sound(rx,sound));
    
    text(&format!("You have engaged in combat with {}",enemy.get_name()));
    enemy.show_stats();
    let mut input=String::new(); 

    match reason{
        _=>(),
    };

    loop{
        
        text(&format!("HP   {}:{}   {}:{}",inventory.player.get_name(),inventory.player.get_health(),enemy.get_name(),enemy.get_health()));
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        check_for_quit(&input);
        match input.trim().to_lowercase().as_str(){
            "inventory"=>inventory.display(),   
            "el psy congroo"=>inventory.player.heal(100),   
            "use"=>{
                let mut item_name=String::new();
                io::stdin().read_line(&mut item_name).unwrap();
                if let Some(index)=inventory.items.iter().position(|x|x.name.trim().to_lowercase()==item_name.trim().to_lowercase()){
                    let item=inventory.items[index].clone(); 

                    match item.item_type{
                        ItemType::CONSUMABLE=>{
                            inventory.player.heal(item.value);
                            inventory.items.remove(index); 
                            play_single_sound(&Sound::NAME("consume.mp3"));
                            text(&format!("Consumed {}",item.name));
                        }
                        ItemType::EQUIPABLE=> {
                            inventory.equipped=item.clone();
                            play_single_sound(&Sound::NAME("equip.mp3"));
                            text(&format!("{} equipped",item.name));
                        }
                    }
                }
                else{text("Item not in inventory");}
            },
            "attack"=>{
                //Player Turn
                let player_strength:i32=match inventory.equipped.name.as_str(){
                    "None"=>{
                        text(&format!("{} attacked",inventory.player.get_name()));
                        *inventory.player.get_strength()as i32
                    },
                    _=>{
                        text(&format!("{} attacked with {}",inventory.player.get_name(),inventory.equipped.name));
                        *inventory.player.get_strength()as i32+inventory.equipped.value 
                    } 
                };                  
                let damage=rng.gen_range(0..=(player_strength-*enemy.get_defense()as i32));
                play_single_sound(&Sound::NAME("hit.mp3"));
                enemy.damage(damage);
                if enemy.get_health()<=0{
                    reason=Reason::VICTORY;
                    break;
                }

                //Enemy Turn
                enemy_attack_text(enemy.get_name());
                play_single_sound(&Sound::NAME("hit.mp3"));
                let damage=rng.gen_range(0..=(*enemy.get_strength()as i32-*inventory.player.get_defense()as i32));
                inventory.player.damage(damage);

                if inventory.player.get_health()<=0{
                    reason=Reason::DEATH;
                    break;
                }
            },
            _=>continue,
        }
        continue;
    }
    let _=tx.send(());
    if let Some(h)=handle{h.join().unwrap();}

    match reason{
        Reason::DEATH=>you_died(),
        Reason::VICTORY=>{
            play_single_sound(&Sound::NAME("enemy_death.mp3"));
            text(&format!("{} has died.",enemy.get_name()))
        },
        Reason::INIT=>()
    }
}

pub fn enemy_attack_text(name:&str){
        match name{
            "Elbron"=>text("Elbron attacked with Copilot"),
            "Cory"=>text("Cory attacked with Cursor"),
            "Max"=>text("Max attacked with BikeShedding"),
            "Dr.Sat"=>text("Dr. Sat attacked with iClicker"),
            "Risc Wrecker"=>text("Risc Wrecker attacked with Seg Fault"),
            _=>()
        }
}
