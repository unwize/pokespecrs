use crate::cache::{del_cache_on_disk, fetch_move_methods, fetch_species_id, get_and_cache_pokemon, get_db_connection, is_cache, is_species_cached, set_up_db};
use crate::console::success;
use crate::enums::Gender;
use crate::errors::{SpecError, SpecErrors};
use crate::spec::{is_learnable_move, PokeSpec};
use crate::{spec, CacheCommands, Commands};
use miette::Result;
use rand::{rng, Rng};
use std::collections::HashMap;

/// A trait that defines the interface for executing command logic
pub trait CommandLogic {
    fn execute(&self, args: Commands) -> Result<()>;
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
    fn execute(&self, args: Commands) -> Result<()> {
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
                    Some(ivs), // TODO: Refactor to allow ivs and evs to both throw errors
                    Some(evs),
                );

                let cache_exists = is_cache();  // Check for cache's existence before opening connection. Creating the conn object automatically initializes db on disk if it doesn't exit.
                let conn = get_db_connection();
                if !cache_exists {
                    set_up_db(&conn).expect("Unable to set up cache!");
                }


                let conn = conn;
                let species_id: i32;
                // TODO: There's probably an easy way to detect if the species is in the cache while retrieving its primary-key id. Rewrite get_species_id?

                if is_species_cached(&conn, species) {
                    species_id = fetch_species_id(&conn, &species)?;
                } else {
                    species_id = get_and_cache_pokemon(species)?;
                }

                let mut errors: Vec<SpecErrors> = Vec::new();

                for poke_move in moveset {
                    let methods = fetch_move_methods(&conn, species_id, poke_move);
                    if methods.is_ok() {
                        match is_learnable_move(species, poke_move, *level, &methods?) {
                            Err(e) => errors.push(e),
                            _ => {}
                        }
                    } else {
                        println!("{:?}", methods?)
                    }
                }

                if !errors.is_empty() || spec.is_err() {
                    if spec.is_err() {
                        errors.append(&mut spec.err().unwrap().causes);
                    }
                    return Err(SpecError { causes: errors })?;
                }

                success(format!("{}", spec?).as_str());
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub struct Cache;

impl CommandLogic for Cache {
    fn execute(&self, args: Commands) -> Result<()> {
        match &args {
            Commands::Cache(cache_args) => {
                let sub_cmd = &cache_args.command;
                match sub_cmd {
                    CacheCommands::Check { species } => Ok(()),
                    CacheCommands::Clear { .. } => {
                        del_cache_on_disk();
                        Ok(())
                    }
                    CacheCommands::Purge { species } => Ok(()),
                    CacheCommands::Validate {} => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }
}
