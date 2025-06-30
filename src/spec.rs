use std::cmp::max;
use crate::enums::Gender;
use std::collections::HashMap;
use std::process::exit;
use num::PrimInt;
use log::log;
use rustemon::model::pokemon::Stat;

struct StatSpread<Prim: PrimInt> {
    map: HashMap<String, Prim>,  // Value of each stat
    stat_limit: Prim,  // Max value allowed per state
    spread_limit: usize,  // Max value of the sum of each state
}

impl<Prim: PrimInt> StatSpread<Prim> {
    pub fn new(stat_limit: Prim, spread_limit: usize) -> Self {
        let spread = StatSpread {
            map: HashMap::from([
                (String::from("atk"), Prim::zero()),
                (String::from("def"), Prim::zero()),
                (String::from("spatk"), Prim::zero()),
                (String::from("spdef"), Prim::zero()),
                (String::from("spd"), Prim::zero()),
                (String::from("hp"), Prim::zero()),
            ]
        ), stat_limit, spread_limit };
        spread
    }

    pub fn validate(&self) -> bool {
        for key in self.map.keys() {
            if *self.map.get(key).unwrap() > self.stat_limit || *self.map.get(key).unwrap() < Prim::zero() {
                return false
            }
        }

        true
    }

    pub fn atk(&self) -> Prim {
        self.map.get("atk").unwrap().clone()
    }

    pub fn spatk(&self) -> Prim {
        self.map.get("spatk").unwrap().clone()
    }

    pub fn def(&self) -> Prim {
        self.map.get("def").unwrap().clone()
    }

    pub fn spdef(&self) -> Prim {
        self.map.get("spdef").unwrap().clone()
    }

    pub fn spd(&self) -> Prim {
        self.map.get("spd").unwrap().clone()
    }

    pub fn hp(&self) -> Prim {
        self.map.get("hp").unwrap().clone()
    }
}

impl StatSpread<u8> {

    /// Create a new IV Stat struct from a list of u8's.
    ///
    /// # Examples
    /// 
    /// ```
    /// let ivs = StatSpread::from(&[31, 25, 13, 22, 1, 16]
    /// assert_eq!(ivs.atk(), 31)
    /// assert_eq!(ivs.def(), 25)
    /// assert_eq!(ivs.spatk(), 13)
    /// assert_eq!(ivs.spdef(), 22)
    /// assert_eq!(ivs.spd(), 1)
    /// assert_eq!(ivs.hp(), 16)
    /// ```
    pub fn from(stats: &[u8]) -> Self {
        if stats.len() != 6 {
            StatSpread {
                map: HashMap::from([
                    (String::from("atk"), stats[0]),
                    (String::from("def"), stats[1]),
                    (String::from("spatk"), stats[2]),
                    (String::from("spdef"), stats[3]),
                    (String::from("spd"), stats[4]),
                    (String::from("hp"), stats[5])
                ]),
                stat_limit: 31,
                spread_limit: 31 * 6,
            }
        } else {
            println!("Stat spread must be created from a slice of length 6. Got {} instead", stats.len());
            exit(-1)
        }
    }
}

struct PokeSpec {
    species: String,
    ability: String,
    level: u8,  // Max of 100
    nickname: Option<String>,
    shiny: bool,
    ot: String,
    tid: usize,
    sid: usize,
    gender: Gender,
    ball: String,
    nature: String,
    ivs: StatSpread<u8>,  // Max of 31 per stat, no actual stat total
    evs: StatSpread<u16>,  // Max of 252 per stat, with a total of 510
}

impl PokeSpec {
    pub fn new(
        species: String, ability: String, level: u8, nickname: Option<String>, shiny: bool,
        ot: String, tid: usize, sid: usize, gender: Gender, ball: String, nature: String,
        ivs: &[u8], evs: &[u16]
    ) -> PokeSpec {
        PokeSpec {
            species,
            ability,
            level,
            nickname,
            shiny,
            ot,
            tid,
            sid,
            gender,
            ball,
            nature,
            ivs: StatSpread::from(ivs),
            evs: StatSpread::from(evs)
        }
    }
}
