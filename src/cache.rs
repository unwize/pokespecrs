use crate::api::pokemon_move::Move;
use std::fs::{create_dir_all, remove_file};

use num_traits::ToPrimitive;
use rusqlite::{Connection, Result};
use std::path::Path;
use std::process::exit;

const CACHE_PATH: &str = ".pokespecrs/";
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

pub fn del_cache_on_disk() {
    let path = Path::new(CACHE_PATH).join(CACHE_FNAME);
    if path.exists() {
        remove_file(path).expect("Failed to delete cache!");
        println!("Cache deleted!");
    }
}
/// Configure the DB for use, from scratch.
///
/// The cache has a few tables, each of which is centered around the `pokemon` table.
/// Each related table links key data elements to a specific pokemon via the `species` foreign key.
pub fn set_up_db(connection: &Connection) -> Result<()> {
    println!("Initializing cache... This will happen only once!");

    connection.execute(
        "CREATE TABLE IF NOT EXISTS pokemon (
                id INTEGER PRIMARY KEY,
                species VARCHAR NOT NULL
            );",
        (),
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS moves (
                id INTEGER PRIMARY KEY,
                name VARCHAR NOT NULL,
                species_id INTEGER NOT NULL,
                method INTEGER NOT NULL,
                level_learned_at INTEGER,
                generation INTEGER NOT NULL,
                FOREIGN KEY(species_id) REFERENCES pokemon(id)
        );",
        (),
    )?;

    Ok(())
}

/// Check if a given species of pokemon has already been cached
pub fn is_species_cached(connection: &Connection, species: &str) -> bool {
    let mut stmt =
        connection.prepare(format!("SELECT 1 FROM pokemon WHERE species = '{species}'").as_str());

    match stmt {
        Ok(mut res) => res.exists([]).unwrap(),
        Err(err) => {
            println!("Failed to execute cache check for species {}", species);
            println!("{}", err);
            false
        }
    }
}

pub fn get_species_id(connection: &Connection, species: &str) -> Result<i32> {
    let mut stmt =
        connection.prepare(format!("SELECT 1 FROM pokemon WHERE species = '{species}'").as_str());

    match stmt {
        Ok(mut res) => res.query_one([], |row| row.get(0)),
        Err(err) => {
            println!("Failed to fetch ID for species {}", species);
            println!("{}", err);
            exit(-1)
        }
    }
}

pub fn insert_pokemon(connection: &Connection, species: &str) -> Result<()> {
    connection
        .execute(
            format!("INSERT INTO pokemon (species) VALUES ('{}');", species).as_str(),
            (),
        )
        .expect("Failed to insert species");

    Ok(())
}

pub fn insert_moves(connection: &Connection, moves: Vec<Move>, species_id: i32) -> Result<()> {
    let mut buffer: Vec<String> = vec![String::from("BEGIN")];

    for pk_move in moves {
        for method in &pk_move.generations {
            buffer.push(format!("INSERT INTO moves (name, species_id, method, level_learned_at, generation) VALUES ('{}', '{}', '{:?}', '{:?}', '{:?}');", pk_move.name, species_id, method.method.to_i32(), method.level_learned_at, method.generation.to_i32()))
        }
    }

    buffer.push(String::from("END;"));

    connection
        .execute(buffer.join("\n").as_str(), ())
        .expect("Failed to batch execute Pokemon Moves INSERT to cache.");
    Ok(())
}
