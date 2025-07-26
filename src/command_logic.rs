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
                let mut spec_builder = PokeSpecBuilder::new(species);
                if ivatk.is_some() {
                    spec_builder.ivs().atk(ivatk.unwrap());
                }
                if ivdef.is_some() {
                    spec_builder.ivs().def(ivdef.unwrap());
                }
                if ivspatk.is_some() {
                    spec_builder.ivs().spatk(ivspatk.unwrap());
                }
                if ivspdef.is_some() {
                    spec_builder.ivs().spdef(ivspdef.unwrap());
                    
                }
                if ivspd.is_some() {
                    spec_builder.ivs().spd(ivspd.unwrap());
                }
                if ivhp.is_some() {
                    spec_builder.ivs().hp(ivhp.unwrap());
                }
                if evatk.is_some() {
                    spec_builder.evs().atk(evatk.unwrap());
                }
                if evdef.is_some() {
                    spec_builder.evs().def(evdef.unwrap());
                }
                if evspatk.is_some() {
                    spec_builder.evs().spatk(evspatk.unwrap());
                }
                if evspdef.is_some() {
                    spec_builder.evs().spdef(evspdef.unwrap());
                }
                if evspd.is_some() {
                    spec_builder.evs().spd(evspd.unwrap());
                }
                if evhp.is_some() {
                    spec_builder.evs().hp(evhp.unwrap());
                }

                let cache_exists = is_cache();  // Check for cache's existence before opening connection. Creating the conn object automatically initializes db on disk if it doesn't exit.
                let conn = get_db_connection();
                if !cache_exists {
                    set_up_db(&conn).expect("Unable to set up cache!");
                }
                
                let spec = spec_builder.build();
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
