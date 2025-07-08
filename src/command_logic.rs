use crate::api::pokemon_move::Move;
use crate::api::{get_pokemon, get_pokemon_moves};
use crate::cache::{
    del_cache_on_disk, get_db_connection, get_species_id, insert_moves, insert_pokemon,
    is_cache_on_disk, is_species_cached, set_up_db,
};
use crate::console::success;
use crate::enums::Gender;
use crate::spec::PokeSpec;
use crate::{CacheCommands, Commands, spec};
use rand::{Rng, rng};
use std::collections::HashMap;

/// A trait that defines the interface for executing command logic
pub trait CommandLogic {
    fn execute(&self, args: Commands);
}

pub struct Generate;

pub struct Cache;

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

                let conn = get_db_connection();
                set_up_db(&conn).expect("Unable to set up cache!");

                let conn = conn;
                let moves: Vec<Move>;
                let species_id: i32;
                // TODO: Unify cache-miss and cache-hit logic for moves. I.E, a cache-miss should result in caching, and then retrieval to simplify overall logic.
                // TODO: There's probably an easy way to detect if the species is in the cache while retrieving its primary-key id. Rewrite get_species_id?

                match is_species_cached(&conn, species) {
                    true => {
                        species_id = get_species_id(&conn, &species).unwrap();
                        moves = vec![];
                    }
                    false => {
                        println!(
                            "Downloading and caching moves... This will only happen once per Pokemon!"
                        );
                        moves = get_pokemon_moves(&get_pokemon(&species));

                        // TODO: Cache these moves, potentially in an independent thread
                        insert_pokemon(&conn, species)
                            .expect("Error when inserting species into cache!");
                        species_id = get_species_id(&conn, &species).unwrap();
                        insert_moves(&conn, moves, species_id)
                            .expect("Error when inserting moves into cache!");
                    }
                }
                success(format!("{spec}").as_str())
            }
            _ => {}
        }
    }
}

impl CommandLogic for Cache {
    fn execute(&self, args: Commands) {
        match &args {
            Commands::Cache(cache_args) => {
                let sub_cmd = &cache_args.command;
                match sub_cmd {
                    CacheCommands::Check { species } => {}
                    CacheCommands::Enable { .. } => {}
                    CacheCommands::Disable { .. } => {}
                    CacheCommands::Clear { .. } => del_cache_on_disk(),
                    CacheCommands::Purge { species } => {}
                    CacheCommands::Validate {} => {}
                }
            }
            _ => {}
        }
    }
}
