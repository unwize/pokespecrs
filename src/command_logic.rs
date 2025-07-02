use crate::api::{get_pokemon, get_pokemon_moves};
use crate::console::{info, success};
use crate::enums::Gender;
use crate::spec::PokeSpec;
use crate::{Commands, spec};
use rand::{Rng, rng};
use std::collections::HashMap;
use itertools::Itertools;
use log::info;

/// A trait that defines the interface for executing command logic
pub trait CommandLogic {
    fn execute(&self, args: Commands);
}

pub struct Generate;

impl CommandLogic for Generate {
    /// Generate a PokeSpec from a given set of arguments.
    /// See: README.md for more.
    ///
    /// Required Args / Args with Defaults
    ///  - species
    ///  - level
    ///  - shiny
    ///  - ot
    ///  - ball
    ///
    /// Optional Args with Random Defaults
    /// - tid
    /// - sid
    /// - gender
    /// - nature
    /// - ivs
    /// - evs
    ///
    /// Optional Args with No Default
    /// - nickname
    /// - moveset
    fn execute(&self, args: Commands) {
        // TODO: Get ability or random ability
        match &args {
            Commands::Generate {
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
                ivatk,
                ivspatk,
                ivdef,
                ivspdef,
                ivspd,
                ivhp,
                evatk,
                evspatk,
                evdef,
                evspdef,
                evspd,
                evhp,
                moveset,
                generation,
            } => {
                let mut ivs: HashMap<String, u16> = HashMap::new();
                if ivatk.is_some() {
                    ivs.insert("atk".to_string(), ivatk.unwrap());
                }
                if ivdef.is_some() {
                    ivs.insert("def".to_string(), ivdef.unwrap());
                }
                if ivspatk.is_some() {
                    ivs.insert("spatk".to_string(), ivspatk.unwrap());
                }
                if ivspdef.is_some() {
                    ivs.insert("spdef".to_string(), ivspdef.unwrap());
                }
                if ivspd.is_some() {
                    ivs.insert("spd".to_string(), ivspd.unwrap());
                }
                if ivhp.is_some() {
                    ivs.insert("hp".to_string(), ivhp.unwrap());
                }

                let mut evs: HashMap<String, u16> = HashMap::new();
                if evatk.is_some() {
                    evs.insert("atk".to_string(), evatk.unwrap());
                }
                if evdef.is_some() {
                    evs.insert("def".to_string(), evdef.unwrap());
                }
                if evspatk.is_some() {
                    evs.insert("spatk".to_string(), evspatk.unwrap());
                }
                if evspdef.is_some() {
                    evs.insert("spdef".to_string(), evspdef.unwrap());
                }
                if evspd.is_some() {
                    evs.insert("spd".to_string(), evspd.unwrap());
                }
                if evhp.is_some() {
                    evs.insert("hp".to_string(), evhp.unwrap());
                }

                let mut rng = rng();
                let spec = PokeSpec::new(
                    species.clone(),
                    "".to_string(),
                    *level,
                    None,
                    *shiny,
                    ot.clone(),
                    tid.unwrap_or(rng.random_range(10000..100000)),
                    sid.unwrap_or(rng.random_range(10000..100000)),
                    Gender::Male,
                    ball.clone().unwrap(),
                    if nature.is_some() {
                        nature.clone().unwrap()
                    } else {
                        spec::NATURES[rng.random_range(0..spec::NATURES.len())].to_string()
                    },
                    Some(ivs),
                    Some(evs),
                );

                let moves = get_pokemon_moves(&get_pokemon(&species));
                info(&format!("{}", moves.iter().format(",")));
                success(format!("{spec}").as_str())
            }
        }
    }
}
