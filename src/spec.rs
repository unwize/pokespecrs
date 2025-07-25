use crate::api::pokemon_move::MoveLearnMethod;
use crate::enums::{Gender, LearnMethod};
use crate::errors::SpecErrors::{EvSumError, EvValueError, IllegalAbilityError, IvValueError, LevelTooLowMoveError, UnlearnableMoveError};
use crate::errors::{SpecError, SpecErrors};
use miette::{Error, Result};
use rusqlite::fallible_iterator::FallibleIterator;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use rand::{rng, Rng};
use crate::cache::{fetch_abilities, fetch_move_methods, fetch_species_id, get_and_cache_pokemon, get_db_connection, is_species_cached};
use crate::console::err;
use crate::util::sample_hash_set;

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
                _stats.insert(stat.clone(), 0);
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
        ivs: StatSpread,
        evs: StatSpread,
    ) -> Self {

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
            ivs,
            evs,
        }
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



pub struct PokeSpecBuilder {
    species: String,
    ability: Option<String>,
    level: u8, // Max of 100
    nickname: Option<String>,
    shiny: bool,
    ot: String,
    tid: usize,
    sid: usize,
    gender: Option<Gender>,
    ball: String,
    nature: Option<String>,
    ivs: StatSpreadBuilder, // Max of 31 per stat, no actual stat total
    evs: StatSpreadBuilder, // Max of 252 per stat, with a total of 510
    move_set: HashSet<String>, // A set of up to four moves denoting the active move set of the pokemon
}

impl PokeSpecBuilder {

    pub fn species(&mut self, species: &str) -> &mut Self {
        self.species = species.to_string();
        self
    }

    pub fn ability(&mut self, ability: &str) -> &mut Self {
        self.ability = Some(ability.to_string());
        self
    }

    pub fn level(&mut self, level: u8) -> &mut Self {
        self.level = level;
        self
    }

    pub fn nickname(&mut self, nickname: &str) -> &mut Self {
        self.nickname = Some(nickname.to_string());
        self
    }

    pub fn shiny(&mut self, shiny: bool) -> &mut Self {
        self.shiny = shiny;
        self
    }

    pub fn ot(&mut self, ot: &str) -> &mut Self {
        self.ot = ot.to_string();
        self
    }

    pub fn tid(&mut self, tid: usize) -> &mut Self {
        self.tid = tid;
        self
    }

    pub fn sid(&mut self, sid: usize) -> &mut Self {
        self.sid = sid;
        self
    }

    pub fn gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    pub fn ball(&mut self, ball: &str) -> &mut Self {
        self.ball = ball.to_string();
        self
    }

    pub fn nature(&mut self, nature: &str) -> &mut Self {
        self.nature = Some(nature.to_string());
        self
    }

    pub fn ivs(&mut self)-> &mut StatSpreadBuilder {
        &mut self.ivs
    }

    pub fn evs(&mut self)-> &mut StatSpreadBuilder {
        &mut self.evs
    }

    pub fn move_set(&mut self, move_set: HashSet<String>) -> &mut Self {
        self.move_set = move_set;
        self
    }

    pub fn new(species: &str) -> Self {
        PokeSpecBuilder {
            species: species.to_string(),
            ability: None, // Either get from user or fill randomly from DB
            level: 1,
            nickname: None,
            shiny: false,
            ot: "PokeSpecRS".to_string(),
            tid: 0,       // TODO: Implement
            sid: 0,       // TODO: Implement
            gender: None, // Either get from user or fill randomly from DB,
            ball: "Poke".to_string(),
            nature: None, // Either get from user or fill randomly from array
            ivs: StatSpreadBuilder::ivs(),
            evs: StatSpreadBuilder::evs(),
            move_set: HashSet::new(),
        }
    }

    pub fn build(&self) -> Result<PokeSpec, Error> {
        let conn = get_db_connection();
        if !is_species_cached(&conn, self.species.as_str()) {
            get_and_cache_pokemon(self.species.as_str())?;
        }

        let species_id = fetch_species_id(&conn, self.species.as_str())?;

        let mut error: Option<SpecError> = None;

        let legal_abilities = fetch_abilities(&conn, species_id)?;

        // Determine legality of the ability. If no ability was provided by the user, randomly select one instead.
        match self.ability.clone() {
            Some(ability) => {
                if !legal_abilities.contains(&ability) {
                    if error.is_none() {
                        error = Some(SpecError {causes: Vec::new()})
                    }
                    error = Some(error.unwrap() + IllegalAbilityError {species: self.species.clone(), ability: ability.clone()});
                }
            }

            None => {}
        }

        // Determine legality of the moveset
        for poke_move in &self.move_set {
            let methods = fetch_move_methods(&conn, species_id, poke_move);
            if methods.is_ok() {
                match is_learnable_move(&self.species, poke_move, self.level, &methods?) {
                    Err(e) => {
                        if error.is_none() {
                            error = Some(SpecError {causes: Vec::new()});
                            error = Some(error.unwrap() + e);
                        }
                    },
                    _ => {}
                }
            } else {
                println!("{:?}", methods?)
            }
        }

        // Determine the legality of the provided gender. If no gender was provided, select one randomly
        if self.gender.is_some() {
            // TODO: Check legality
        } else {

        }

        // Check if IVs or EVs have any errors. If so, accumulate them in the `error` field.
        let ivs = self.ivs.build();
        if ivs.is_err() {
            error = Some(ivs.clone().err().unwrap() + error);
        }

        let evs = self.evs.build();
        if evs.is_err() {
            error = Some(evs.clone().err().unwrap() + error);
        }

        if error.is_some() {
            return Err(error.unwrap())?;
        }

        Ok(PokeSpec::new(
            self.species.clone(),
            self.ability.clone().unwrap_or(sample_hash_set(&fetch_abilities(&conn, species_id)?)),
            self.level,
            self.nickname.clone(),
            self.shiny,
            self.ot.clone(),
            self.tid,
            self.sid,
            self.gender.clone().unwrap_or(Gender::Genderless),
            self.ball.clone(),
            self.nature.clone().unwrap_or(NATURES.get(rng().random_range(0..NATURES.len())).unwrap().to_string()),
            ivs?,
            evs?
        ))


    }
}

pub fn is_learnable_move(
    species: &str,
    pk_move: &str,
    pk_level: u8,
    methods: &HashSet<MoveLearnMethod>,
) -> Result<(), SpecErrors> {
    // No methods mean the move is not learnable at all
    if methods.len() < 1 {
        Err(UnlearnableMoveError {
            species: String::from(species),
            pk_move: String::from(pk_move),
        })?
    }

    let mut min_learn_level: Option<u8> = None;
    for method in methods {
        // Alternative learn methods mean the move is learnable regardless of level
        if [LearnMethod::Egg, LearnMethod::Machine, LearnMethod::Tutor].contains(&method.method) {
            return Ok(());
        }

        // Level-based learning must work number-wise, else move can't be learned at all
        if method.method == LearnMethod::LevelUp {
            let method_level = method.level_learned_at.clone().unwrap();
            if method_level <= pk_level {
                return Ok(());
            }

            if min_learn_level == None {
                min_learn_level = Some(method_level);
            } else {
                let lvl = min_learn_level.unwrap();
                if method_level < lvl {
                    min_learn_level = Some(method_level);
                } else {
                    min_learn_level = Some(lvl);
                }
            }
        }
    }

    Err(LevelTooLowMoveError {
        species: String::from(species),
        pk_move: String::from(pk_move),
        level: pk_level.to_string(),
        min_level: min_learn_level.unwrap().to_string(),
    })?
}
