use std::cmp::min;
use std::collections::{HashMap, HashSet};
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
    fn new(stat_max: u16, sum_max: u16, user_stats: HashMap<String, u16>) -> Self {
        let mut available_stats: HashSet<String> = HashSet::from(STAT_NAMES.clone().map(|x| {x.to_string()}));
        let mut _stats: HashMap<String, u16> = HashMap::new();
        let mut rng = rand::rng();
        let mut sum: u16 = 0;

        for stat in available_stats {
            if user_stats.contains_key(&stat) {
                let value = *user_stats.get(&stat).unwrap();
                sum = sum + value;
                _stats.insert(stat.clone(), value);
            } else {
                let value =  rng.random_range(0..min(stat_max, sum_max - sum) + 1);
                sum = sum + value;
                _stats.insert(stat.clone(), value);
            }
        }

        StatSpread {
            stats: _stats,
            stat_max,
            sum_max,
        }
    }

    fn from_ivs(stats: Option<HashMap<String, u16>>) -> Self {
        let mut available_stats = HashSet::from(STAT_NAMES.clone());
        let mut _stats: HashMap<String, u16> = HashMap::new();

        if stats.is_some() {
            for (stat, value) in stats.unwrap() {
                if STAT_NAMES.contains(&stat.as_str()) {
                    if *value > 31 || *value < 0 {
                        // TODO: Handle invalid stat value
                    }

                    available_stats.remove(stat.as_str());
                    _stats.insert(stat.clone(), *value);
                } else {
                    // TODO: Handle invalid stat name
                }
            }
        }

        let mut rng = rand::rng();
        for stat in available_stats {
            _stats.insert(String::from(stat), rng.random_range(0..32));
        }

        StatSpread {
            stats: _stats,
            stat_max: 31,
            sum_max: 31 * 6,
        }
    }

    fn from_evs(stats: Option<HashMap<String, u16>>) -> Self {
        let mut available_stats = HashSet::from(STAT_NAMES.clone());
        let mut _stats: HashMap<String, u16> = HashMap::new();
        let mut sum: u16 = 0;

        if !stats.is_none() {
            // Pull user-defined stat values out of the provided hashmap
            for (stat, value) in stats.unwrap() {
                if STAT_NAMES.contains(&stat.as_str()) {

                    // Remove the stat from the set of available stats
                    available_stats.remove(stat.as_str());

                    if sum + *value > 510 {
                        // TODO: Handle max sum overflow
                    }

                    if *value > 252 {
                        // TODO: Handle max value overflow
                    }
                    sum = sum + *value;

                    _stats.insert(String::from(stat), *value);
                } else {
                    // TODO: Handle invalid stat name
                }
            }
        }

        // For each stat that was not provided, generate a random value for it
        let mut rng = rand::rng();
        for stat in available_stats {
            let value = rng.random_range(0..min(252, 510-sum) + 1);  // The stat's value has a max possible value of 252. The value may also not excede a stat sum total of 510.
            _stats.insert(String::from(stat), value);
            sum = sum + value;
        }

        StatSpread {
            stats: _stats,
            stat_max: 252,
            sum_max: 510,
        }
    }
}


pub struct PokeSpec {
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
        ivs: Option<HashMap<String, u16>>, evs: Option<HashMap<String, u16>>
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
            ivs: StatSpread::from_ivs(ivs),
            evs: StatSpread::from_evs(evs)
        }
    }
}
