use global_hotkey::{GlobalHotKeyManager, hotkey::HotKey, GlobalHotKeyEvent};
use keyboard_types::{Code, Modifiers};

pub struct HotkeysConfig{
    hotkeys: Vec<HotKey>,
    hotkeys_seq: Vec<(Option<Modifiers>, Code)>,
    hotkeys_string: Vec<(String, String)>,
    commands: Vec<String>,
    new_key: (Code, String), //new key to be decided
    new_mod: (Option<Modifiers>, String), //new modifier to be decided
    enable: bool,  //used to enable the modification of a hotkey
    changed_hotkey: usize, //what hotkey do I want to change
    manager: GlobalHotKeyManager,
}

impl HotkeysConfig{
    pub fn new() -> HotkeysConfig{
        let h:Vec<HotKey> = vec![HotKey::new(Some(Modifiers::SHIFT), Code::KeyD), HotKey::new(Some(Modifiers::SHIFT), Code::KeyA)];
        let j:Vec<(Option<Modifiers>, Code)> = vec![(Some(Modifiers::SHIFT), Code::KeyD), (Some(Modifiers::SHIFT), Code::KeyA)];
        let com:Vec<String> = vec!["Take screenshot".to_string(), "Save".to_string()];
        let man = GlobalHotKeyManager::new().unwrap();
        man.register_all(&h).unwrap(); //Registering default hotkeys

        HotkeysConfig { hotkeys: h, hotkeys_seq: j, hotkeys_string: vec![("SHIFT".to_string(), "D".to_string()), ("SHIFT".to_string(), "A".to_string())], new_key: (Code::KeyA, "A".to_string()), new_mod: (Some(Modifiers::SHIFT), "SHIFT".to_string()), enable: true, changed_hotkey:0, commands: com, manager: man}
    }

    pub fn get_new_key(self: &Self) -> (Code, String){
        return self.new_key.clone();
    }

    pub fn get_new_mod(self: &Self) -> (Option<Modifiers>, String){
        return self.new_mod.clone();
    }

    pub fn get_changed_hotkey(self: &Self) -> usize{
        return self.changed_hotkey;
    }

    pub fn get_enable(self: &Self) -> bool{
        return self.enable;  
    }

    pub fn set_new_hotkey(self: &mut Self, new_mod:(Option<Modifiers>, String), new_key: (Code, String)){
        self.new_mod = new_mod;
        self.new_key = new_key;
    }

    pub fn set_enable(self: &mut Self, en: bool){
        self.enable = en;
    }

    pub fn get_hotkeys_len(self: &Self) -> usize{
        return self.hotkeys.len();
    }

    pub fn get_command(self: &Self, i: usize) -> &String{
        return &self.commands[i];
    }

    // pub fn get_hotkey(self: &Self, i:usize) -> (&str, &str){
    //     return (self.hotkeys_string[i].0.as_str(), self.hotkeys_string[i].1.as_str());
    // }

    pub fn get_hotkey_as_string(self: &Self, i:usize) -> String{

        if self.hotkeys_string[i].0.eq(""){
            return self.hotkeys_string[i].1.clone();
        }
        else{
            return self.hotkeys_string[i].0.clone() + "+" + &self.hotkeys_string[i].1;
        }
    }

    pub fn listen_to_event(self:&Self){
        if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            println!("tray event: {event:?}");
            println!("{:?}", self.hotkeys_seq);
            println!("{:?}", self.commands);
        }
    }

    pub fn delete_hotkey(self: &mut Self, i:usize){
        self.manager.unregister(self.hotkeys[i]).unwrap();
        self.new_mod=(self.hotkeys_seq[i].0, self.hotkeys_string[i].0.clone());
        self.new_key=(self.hotkeys_seq[i].1, self.hotkeys_string[i].1.clone());
        self.changed_hotkey = i;
        self.enable = false;
    }

    pub fn change_hotkey(self: &mut Self, i: usize, modifier: (Option<Modifiers>, String), key: (Code, String)) -> bool{
        for c in 0..self.hotkeys_string.len(){
            if self.hotkeys_string[c].0.eq(&modifier.1) && self.hotkeys_string[c].1.eq(&key.1) && c != i {
                return false;
            }
        }
        self.hotkeys_string[i]=(modifier.1, key.1);
        self.hotkeys_seq[i]=(modifier.0, key.0);
        self.hotkeys[i] = HotKey::new(modifier.0, key.0);
        self.manager.register(self.hotkeys[i]).unwrap();
        return true;
    }
}