use crate::enums::Gender;
use crate::errors::SpecErrors::{EvSumError, EvValueError, IvValueError};
use crate::errors::{SpecError, SpecErrors};
use miette::Result;
use rand::Rng;
use rusqlite::fallible_iterator::FallibleIterator;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

static STAT_NAMES: [&str; 6] = ["atk", "def", "spatk", "spdef", "spd", "hp"];
pub static NATURES: [&str; 25] = [
    "Hardy", "Lonely", "Adamant", "Naughty", "Brave", "Bold", "Docile", "Impish", "Lax", "Relaxed",
    "Modest", "Mild", "Bashful", "Rash", "Quiet", "Calm", "Gentle", "Careful", "Quirky", "Sassy",
    "Timid", "Hasty", "Jolly", "Naive", "Serious",
];

pub struct StatSpreadBuilder {
    stat_max: u16,
    sum_max: u16,
    stats: HashMap<String, u16>,
    stat_type: Option<StatSpreadType>,
}

#[derive(Clone, Debug)]
pub enum StatSpreadType {
    IV = 0,
    EV = 1,
}

impl StatSpreadBuilder {
    fn new(stat_max: u16, sum_max: u16) -> Self {
        StatSpreadBuilder {
            stat_max,
            sum_max,
            stats: HashMap::new(),
            stat_type: None,
        }
    }

    fn ivs() -> Self {
        StatSpreadBuilder {
            stat_max: 31,
            sum_max: 31 * 6,
            stats: HashMap::new(),
            stat_type: Some(StatSpreadType::IV),
        }
    }

    fn evs() -> Self {
        StatSpreadBuilder {
            stat_max: 252,
            sum_max: 510,
            stats: HashMap::new(),
            stat_type: Some(StatSpreadType::EV),
        }
    }

    fn stat_max(&mut self, max: u16) -> &mut Self {
        self.stat_max = max;
        self
    }

    fn sum_max(&mut self, max: u16) -> &mut Self {
        self.sum_max = max;
        self
    }

    fn atk(&mut self, atk: u16) -> &mut Self {
        self.stats.insert("atk".to_string(), atk);
        self
    }

    fn def(&mut self, def: u16) -> &mut Self {
        self.stats.insert("def".to_string(), def);
        self
    }

    fn spatk(&mut self, spatk: u16) -> &mut Self {
        self.stats.insert("spatk".to_string(), spatk);
        self
    }

    fn spdef(&mut self, spdef: u16) -> &mut Self {
        self.stats.insert("spdef".to_string(), spdef);
        self
    }

    fn spd(&mut self, spd: u16) -> &mut Self {
        self.stats.insert("spd".to_string(), spd);
        self
    }

    fn hp(&mut self, hp: u16) -> &mut Self {
        self.stats.insert("hp".to_string(), hp);
        self
    }

    fn stats(&mut self, stats: HashMap<String, u16>) -> &mut Self {
        self.stats = stats;
        self
    }

    pub fn build(&self) -> Result<StatSpread, SpecError> {
        StatSpread::new(
            self.stat_max,
            self.sum_max,
            self.stats.clone(),
            self.stat_type.clone().unwrap(),
        )
    }
}

#[derive(Clone, Debug)]
struct StatSpread {
    pub stats: HashMap<String, u16>,
    stat_max: u16,
    sum_max: u16,
    stat_type: StatSpreadType,
}

impl StatSpread {
    fn new(
        stat_max: u16,
        sum_max: u16,
        user_stats: HashMap<String, u16>,
        stat_type: StatSpreadType,
    ) -> Result<Self, SpecError> {
        let available_stats: HashSet<String> =
            HashSet::from(STAT_NAMES.clone().map(|x| x.to_string()));
        let mut _stats: HashMap<String, u16> = HashMap::new();
        let mut rng = rand::rng();
        let mut sum: u16 = 0;

        let mut errors: Vec<SpecErrors> = Vec::new();

        for stat in available_stats {
            if user_stats.contains_key(&stat) {
                let value = *user_stats.get(&stat).unwrap();

                if value > stat_max {
                    match stat_type {
                        StatSpreadType::IV => errors.push(IvValueError {
                            stat,
                            value: value.to_string(),
                        }),
                        StatSpreadType::EV => errors.push(EvValueError {
                            stat,
                            value: value.to_string(),
                        }),
                    }
                    continue;
                }

                if sum + value > sum_max {
                    errors.push(EvSumError {
                        ev_sum: (sum + value).to_string(),
                    });
                    break;
                }

                sum = sum + value;
                _stats.insert(stat.clone(), value);
            } else {
                let value = rng.random_range(0..min(stat_max, sum_max - sum) + 1);
                sum = sum + value;
                _stats.insert(stat.clone(), value);
            }
        }

        if errors.len() > 0 {
            return Err(SpecError { causes: errors });
        }

        Ok(StatSpread {
            stats: _stats,
            stat_max,
            sum_max,
            stat_type,
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
    species: String,
    ability: String,
    level: u8, // Max of 100
    nickname: Option<String>,
    shiny: bool,
    ot: String,
    tid: usize,
    sid: usize,
    gender: Gender,
    ball: String,
    nature: String,
    ivs: StatSpread, // Max of 31 per stat, no actual stat total
    evs: StatSpread, // Max of 252 per stat, with a total of 510
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
    ) -> Result<Self, SpecError> {
        let mut ivs = StatSpreadBuilder::ivs()
            .stats(ivs.unwrap_or(HashMap::new()))
            .build();
        let mut evs = StatSpreadBuilder::evs()
            .stats(evs.unwrap_or(HashMap::new()))
            .build();

        // TODO: There must be a cleaner way of doing this
        if ivs.is_err() && evs.is_err() {
            return Err(ivs.err().unwrap() + evs.err().unwrap());
        } else if ivs.is_err() {
            return Err(ivs.err().unwrap());
        } else if evs.is_err() {
            return Err(evs.err().unwrap());
        }

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
            ivs: ivs?,
            evs: evs?,
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
