use crate::api::pokemon_move::{Move, MoveLearnMethod};
use std::collections::HashSet;
use std::fs::{create_dir_all, remove_file};

use crate::enums::{Generation, LearnMethod};
use num_traits::{FromPrimitive, ToPrimitive};
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
                species VARCHAR NOT NULL COLLATE NOCASE
            );",
        (),
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS moves (
                id INTEGER PRIMARY KEY,
                name VARCHAR NOT NULL COLLATE NOCASE,
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
    let stmt =
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
    let stmt =
        connection.prepare(format!("SELECT * FROM pokemon WHERE species = '{species}'").as_str());

    match stmt {
        Ok(mut res) => res.query_one([], |row| row.get(0)),
        Err(err) => {
            // TODO: Is this a candidate for a miette error?
            println!("Failed to fetch ID for species {}", species);
            println!("{}", err);
            exit(-1)
        }
    }
}

pub fn insert_pokemon(connection: &Connection, species: &str) -> Result<()> {
    let stmt = connection.execute(
        format!("INSERT INTO pokemon (species) VALUES ('{}');", species).as_str(),
        (),
    );

    match stmt {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn insert_moves(connection: &Connection, moves: &Vec<Move>, species_id: i32) -> Result<()> {
    let mut buffer: Vec<String> = vec![String::from("BEGIN;")];

    for pk_move in moves {
        for method in &pk_move.generations {
            buffer.push(format!("INSERT INTO moves (name, species_id, method, level_learned_at, generation) VALUES ('{}', '{}', '{}', '{}', '{}');", pk_move.name, species_id, method.method.to_i32().unwrap(), method.level_learned_at.unwrap(), method.generation.to_i32().unwrap()))
        }
    }

    buffer.push(String::from("COMMIT;"));

    let res = connection.execute_batch(buffer.join(" ").as_str());
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err), // Pass error up
    }
}

pub fn fetch_move_methods(
    conn: &Connection,
    species_id: i32,
    move_name: &str,
) -> Result<HashSet<MoveLearnMethod>> {
    let stmt = conn.prepare("SELECT * FROM moves WHERE species_id = ?1 and name = ?2;");

    match stmt {
        Ok(mut res) => {
            let moves_sql = res.query(rusqlite::params![species_id, move_name]);
            let mut moves: HashSet<MoveLearnMethod> = HashSet::new();

            match moves_sql {
                // Yes, I know I'm shadowing the variable in the outer scope. It's fine.
                Ok(mut moves_sql) => {
                    // Must use weird next() interface as Rows object does not implement Iterator trait
                    while let Some(row) = moves_sql.next()? {
                        // idx corresponds to the order in which columns are declared in table creation statement
                        moves.insert(MoveLearnMethod {
                            method: LearnMethod::from_i32(row.get(3)?).unwrap(),
                            level_learned_at: row.get(4)?,
                            generation: Generation::from_i32(row.get(5)?).unwrap(),
                        });
                    }
                    Ok(moves)
                }
                Err(e) => Err(e), // Pass the error up
            }
        }
        Err(e) => Err(e), // Pass the error up
    }
}
