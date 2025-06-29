use crate::enums::Gender;
use std::collections::HashMap;

struct IVSpec {
    map: HashMap<String, u8>
}

impl IVSpec {
    pub fn new() -> IVSpec {
        let mut ivs = IVSpec { map: HashMap::new() };
        ivs.map.insert(String::from("atk"), 0);
        ivs.map.insert(String::from("spatk"), 0);
        ivs.map.insert(String::from("def"), 0);
        ivs.map.insert(String::from("spdef"), 0);
        ivs.map.insert(String::from("spd"), 0);
        ivs.map.insert(String::from("hp"), 0);
        ivs
    }

    pub fn atk(&self) -> u8 {
        self.map.get("atk").unwrap().clone()
    }

    pub fn spatk(&self) -> u8 {
        self.map.get("spatk").unwrap().clone()
    }

    pub fn def(&self) -> u8 {
        self.map.get("def").unwrap().clone()
    }

    pub fn spdef(&self) -> u8 {
        self.map.get("spdef").unwrap().clone()
    }

    pub fn spd(&self) -> u8 {
        self.map.get("spd").unwrap().clone()
    }

    pub fn hp(&self) -> u8 {
        self.map.get("hp").unwrap().clone()
    }
}

struct EVSpec {
    atk: u8,
    spatk: u8,
    def: u8,
    spdef: u8,
    hp: u8,
    spd: u8
}

struct PokeSpec {
    species: String,
    ability: String,
    level: u8,
    nickname: Option<String>,
    shiny: bool,
    ot: String,
    tid: usize,
    sid: usize,
    gender: Gender,
    ball: String,
    nature: String,

}