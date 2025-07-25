use crate::cache::{del_cache_on_disk, fetch_move_methods, fetch_species_id, get_and_cache_pokemon, get_db_connection, is_cache, is_species_cached, set_up_db};
use crate::console::success;
use crate::enums::Gender;
use crate::errors::{SpecError, SpecErrors};
use crate::spec::{is_learnable_move, PokeSpec, PokeSpecBuilder};
use crate::{spec, CacheCommands, Commands};
use miette::Result;
use rand::{rng, Rng};
use std::collections::HashMap;
use rusqlite::fallible_iterator::FallibleIterator;

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

                let mut spec_builder = PokeSpecBuilder::new(species);
                let spec = spec_builder.build();

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
