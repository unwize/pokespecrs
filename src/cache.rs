use std::fs::{create_dir, create_dir_all};
use crate::api::pokemon_move::{Move};

use rusqlite::{Connection, Result};
use std::path::Path;

const CACHE_PATH: &str = "~/.pokespecrs/";
const CACHE_FNAME: &str = "cache.db3";


/// Create a re-useable connection to the cache DB.
pub fn get_db_connection() -> Connection {
    create_dir_all(Path::new(CACHE_PATH)).expect("Failed to create dir for cache!");
    Connection::open(Path::new(CACHE_PATH).join(CACHE_FNAME)).expect("Failed to connect to cache!")
}

/// Quick and dirty way to see if a cache has already been created
pub fn is_cache_on_disk() -> bool {
    Path::new(CACHE_PATH).exists()
}


/// Configure the DB for use, from scratch.
///
/// The cache has a few tables, each of which is centered around the `pokemon` table.
/// Each related table links key data elements to a specific pokemon via the `species` foreign key.
pub fn set_up_db(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS pokemon (\
                id INTEGER PRIMARY KEY, \
                species VARCHAR NOT NULL\
            )",
        (),
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS moves (\
                id INTEGER PRIMARY KEY, \
                name VARCHAR NOT NULL, \
                FOREIGN KEY (species) REFERENCES pokemon (id),\
                method INTEGER NOT NULL,
                level_learned_at INTEGER,
                generation INTEGER NOT NULL,
        )",
        ())?;

    Ok(())
}

/// Check if a given species of pokemon has already been cached
pub fn is_species_cached(connection: &Connection, species: &str) -> bool {
    let mut stmt = connection.prepare(
        format!("SELECT 1 FROM pokemon WHERE species = '{species}'").as_str()
    );

    match stmt {
        Ok(mut res) => {
            res.exists([]).unwrap()
        }
        Err(err) => {
            println!("Failed to execute cache check for species {}", species);
            println!("{}", err);
            false
        }
    }
}

pub fn insert_pokemon(connection: &Connection, species: &str) -> Result<()> {

    connection.execute(format!("INSERT INTO pokemon (species) VALUES ('{}');", species).as_str(), ()).expect("Failed to insert species");

    Ok(())
}

pub fn insert_moves(connection: &Connection, moves: Vec<Move>) -> Result<()> {


    Ok(())
}
