use crate::enums::Gender;

trait Validate {
    fn validate(&self) -> Result<(), String>;
}

struct IVSpec {
    atk: u8,
    spatk: u8,
    def: u8,
    spdef: u8,
    hp: u8,
    spd: u8
}

impl Validate for IVSpec {
    fn validate(&self) {
        if 0 > self.atk  ||  self.atk > 31 {

        }
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