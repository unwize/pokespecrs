use std::collections::HashMap;
use rand::Rng;
use crate::enums::Gender;

static STAT_NAMES: [&str; 6] = ["atk", "def", "spatk", "spdef", "spd", "hp"];

struct StatSpread {
    stats: HashMap<String, u16>,
    stat_max: u16,
    sum_max: u16,
}

impl StatSpread {

    fn default() -> Self {
        StatSpread {
            stats: HashMap::from(STAT_NAMES.map(|s| { (String::from(s), 0) })),  // For each stat name, map it to a tuple of (stat_name, 0) and generate a HashMap from those k,v pairs.
            stat_max: 0,
            sum_max: 0,
        }
    }
    fn new(stat_max: u16, sum_max: u16, user_stats: Option<&HashMap<String, u16>>) -> Self {

        let mut stats: HashMap<String, u16> = HashMap::new();
        let mut rng = rand::rng();
        for stat in STAT_NAMES {
            if user_stats.contains_key(stat) {
                stats.insert(String::from(stat), *user_stats.get(stat).unwrap());
            } else {
                stats.insert(String::from(stat), rng.random_range(0..stat_max + 1));
            }
        }



        StatSpread {
            stats,
            stat_max,
            sum_max,
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
    ivs: StatSpread,  // Max of 31 per stat, no actual stat total
    evs: StatSpread,  // Max of 252 per stat, with a total of 510
}

impl PokeSpec {
    pub fn new(
        species: String, ability: String, level: u8, nickname: Option<String>, shiny: bool,
        ot: String, tid: usize, sid: usize, gender: Gender, ball: String, nature: String,
        ivs: Option<&HashMap<String, u16>>, evs: &[u16]
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
