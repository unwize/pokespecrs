use crate::enums::Gender;
use rand::Rng;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use miette::Result;
use crate::errors::{IVValueError, NoSuchStatError, SpecError};

static STAT_NAMES: [&str; 6] = ["atk", "def", "spatk", "spdef", "spd", "hp"];
pub static NATURES: [&str; 25] = [
    "Hardy", "Lonely", "Adamant", "Naughty", "Brave", "Bold", "Docile", "Impish", "Lax", "Relaxed",
    "Modest", "Mild", "Bashful", "Rash", "Quiet", "Calm", "Gentle", "Careful", "Quirky", "Sassy",
    "Timid", "Hasty", "Jolly", "Naive", "Serious",
];

#[derive(Clone, Debug)]
struct StatSpread {
    pub stats: HashMap<String, u16>,
    stat_max: u16,
    sum_max: u16,
}

impl StatSpread {
    fn default() -> Self {
        StatSpread {
            stats: HashMap::from(STAT_NAMES.map(|s| (String::from(s), 0))), // For each stat name, map it to a tuple of (stat_name, 0) and generate a HashMap from those k,v pairs.
            stat_max: 0,
            sum_max: 0,
        }
    }
    fn new(stat_max: u16, sum_max: u16, user_stats: HashMap<String, u16>) -> Self {
        let available_stats: HashSet<String> =
            HashSet::from(STAT_NAMES.clone().map(|x| x.to_string()));
        let mut _stats: HashMap<String, u16> = HashMap::new();
        let mut rng = rand::rng();
        let mut sum: u16 = 0;

        for stat in available_stats {
            if user_stats.contains_key(&stat) {
                let value = *user_stats.get(&stat).unwrap();
                sum = sum + value;
                _stats.insert(stat.clone(), value);
            } else {
                let value = rng.random_range(0..min(stat_max, sum_max - sum) + 1);
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

    fn from_ivs(stats: Option<HashMap<String, u16>>) -> Result<Self> {
        let mut available_stats = HashSet::from(STAT_NAMES.clone());
        let mut _stats: HashMap<String, u16> = HashMap::new();

        if stats.is_some() {
            let mut errors: Vec<dyn Error> = Vec::new();

            for (stat, value) in stats.unwrap() {
                if STAT_NAMES.contains(&stat.as_str()) {
                    if value > 31 {

                        // Collect errors in a vector and return them underneath a SpecError later
                        errors.push(
                            IVValueError {
                                stat: stat.clone(),
                                value
                            }
                        );
                    }

                    available_stats.remove(stat.as_str());
                    _stats.insert(stat.clone(), value);
                } else {
                    errors.push(
                        NoSuchStatError {
                            stat: stat.clone()
                        }
                    );
                }
            }

            if errors.len() > 0 {
                return Err(
                    SpecError {
                        related: errors,
                    }
                )
            }
        }


        let mut rng = rand::rng();
        for stat in available_stats {
            _stats.insert(String::from(stat), rng.random_range(0..32));
        }

        Ok(StatSpread {
            stats: _stats,
            stat_max: 31,
            sum_max: 31 * 6,
        })
    }

    fn from_evs(stats: Option<HashMap<String, u16>>) -> Result<Self> {
        let mut available_stats = HashSet::from(STAT_NAMES.clone());
        let mut _stats: HashMap<String, u16> = HashMap::new();
        let mut sum: u16 = 0;

        if !stats.is_none() {
            // Pull user-defined stat values out of the provided hashmap
            for (stat, value) in stats.unwrap() {
                if STAT_NAMES.contains(&stat.as_str()) {
                    // Remove the stat from the set of available stats
                    available_stats.remove(stat.as_str());

                    if sum + value > 510 {
                        // EV Sum exceeds limit
                    }

                    if value > 252 {
                        // Individual EV value exceeds limit
                    }
                    sum = sum + value;

                    _stats.insert(String::from(stat), value);
                } else {
                    // Invalid stat name
                }
            }
        }

        // For each stat that was not provided, set to zero
        for stat in available_stats {
            _stats.insert(String::from(stat), 0);
        }

        Ok(StatSpread {
            stats: _stats,
            stat_max: 252,
            sum_max: 510,
        })
    }
}

impl Display for StatSpread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "atk: {}, def: {}, spatk: {}, spdef {}, spd: {}, hp: {}",
            self.stats["atk"],
            self.stats["def"],
            self.stats["spatk"],
            self.stats["spdef"],
            self.stats["spd"],
            self.stats["hp"]
        )
    }
}

#[derive(Clone, Debug)]
pub struct PokeSpec {
    pub species: String,
    pub ability: String,
    pub level: u8, // Max of 100
    pub nickname: Option<String>,
    pub shiny: bool,
    pub ot: String,
    pub tid: usize,
    pub sid: usize,
    pub gender: Gender,
    pub ball: String,
    pub nature: String,
    pub ivs: StatSpread, // Max of 31 per stat, no actual stat total
    pub evs: StatSpread, // Max of 252 per stat, with a total of 510
}

impl PokeSpec {
    pub fn new(
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
        ivs: Option<HashMap<String, u16>>,
        evs: Option<HashMap<String, u16>>,
    ) -> Result<Self> {
        Ok(PokeSpec {
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
            ivs: StatSpread::from_ivs(ivs)?,
            evs: StatSpread::from_evs(evs)?,
        })
    }
}

impl Display for PokeSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] ({}), lvl: {} | {} | IVs: {} | EVs: {} | {} |",
            self.species,
            if self.nickname.is_some() {
                self.nickname.clone().unwrap()
            } else {
                self.species.clone()
            },
            self.level,
            self.ability,
            self.ivs,
            self.evs,
            self.nature
        )
    }
}
