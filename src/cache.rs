use std::fs::{create_dir, create_dir_all};
use crate::api::pokemon_move::{Move};

use rusqlite::{Connection, Result};
use std::path::Path;
use std::process::exit;

const CACHE_PATH: &str = "~/.pokespecrs/";
const CACHE_FNAME: &str = "cache.db3";

const POKEMON_TABLE: &str = "pokemon";
const PT_SPECIES_COL: &str = "species";
const MOVE_TABLE: &str = "moves";



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
    connection.execute(format!(
        "CREATE TABLE IF NOT EXISTS {} (\
                id INTEGER PRIMARY KEY, \
                {} VARCHAR NOT NULL\
            )", POKEMON_TABLE, PT_SPECIES_COL).as_str(),
        (),
    )?;

    connection.execute(format!(
        "CREATE TABLE IF NOT EXISTS moves (\
                id INTEGER PRIMARY KEY, \
                name VARCHAR NOT NULL, \
                FOREIGN KEY ({}) REFERENCES {} (id),\
                method INTEGER NOT NULL,
                level_learned_at INTEGER,
                generation INTEGER NOT NULL,
        )", PT_SPECIES_COL, POKEMON_TABLE).as_str(),
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

pub fn get_species_id(connection: &Connection, species: &str) -> Result<i32> {
    let mut stmt = connection.prepare(
        format!("SELECT 1 FROM pokemon WHERE species = '{species}'").as_str()
    );

    match stmt {
        Ok(mut res) => {
            res.query_one([], |row| {row.get("id")})
        }
        Err(err) => {
            println!("Failed to fetch ID for species {}", species);
            println!("{}", err);
            exit(-1)
        }
    }
}

pub fn insert_pokemon(connection: &Connection, species: &str) -> Result<()> {

    connection.execute(format!("INSERT INTO pokemon (species) VALUES ('{}');", species).as_str(), ()).expect("Failed to insert species");

    Ok(())
}

pub fn insert_moves(connection: &Connection, moves: Vec<Move>, species: &str) -> Result<()> {
        let mut buffer: Vec<String> = vec![String::from("BEGIN")];

        for pk_move in moves {
        }

    Ok(())
}
