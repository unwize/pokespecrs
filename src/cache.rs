use crate::api::pokemon_move::{MoveLearnMethod, PokeMove};
use std::collections::HashSet;
use std::fs::{create_dir_all, remove_file};

use crate::api::{api_get_pokemon, api_get_pokemon_abilities, api_get_pokemon_moves};
use crate::enums::{Generation, LearnMethod};
use miette::{Error, ErrorHook, IntoDiagnostic, Result};
use num_traits::{FromPrimitive, ToPrimitive};
use rusqlite::Connection;
use std::path::Path;
use std::process::exit;
use crate::console::info;

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

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS pokemon (
                id INTEGER PRIMARY KEY,
                species VARCHAR NOT NULL COLLATE NOCASE
            );",
            (),
        )
        .into_diagnostic()?;

    connection
        .execute(
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
        )
        .into_diagnostic()?;

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS abilities (
            id INTEGER PRIMARY KEY,\
            name VARCHAR NOT NULL COLLATE NOCASE,\
             species_id INTEGER NOT NULL,
            FOREIGN KEY(species_id) REFERENCES pokemon(id)
        );",
            (),
        )
        .into_diagnostic()?;

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS balls (id INTEGER PRIMARY KEY,\
        name VARCHAR NOT NULL COLLATE NOCASE);",
            (),
        )
        .into_diagnostic()?;

    Ok(())
}

/// Checks if the cache is present on disk. Does not verify cache integrity.
pub fn is_cache() -> bool {
    Path::new(CACHE_PATH).join(CACHE_FNAME).exists()
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

pub fn fetch_species_id(connection: &Connection, species: &str) -> Result<i32> {
    let stmt =
        connection.prepare(format!("SELECT * FROM pokemon WHERE species = '{species}'").as_str());

    match stmt {
        Ok(mut res) => res.query_one([], |row| row.get(0)).into_diagnostic(),
        Err(err) => {
            // TODO: Is this a candidate for a miette error?
            println!("Failed to fetch ID for species {}", species);
            println!("{}", err);
            exit(-1)
        }
    }
}

/// Insert the given species of Pokemon into the `pokemon` table in the cache
pub fn cache_species(connection: &Connection, species: &str) -> Result<()> {
    let stmt = connection.execute(
        format!("INSERT INTO pokemon (species) VALUES ('{}');", species).as_str(),
        (),
    );

    match stmt {
        Ok(_) => Ok(()),
        Err(err) => Err(err).into_diagnostic(),
    }
}

/// For a given species and vector of moves, insert the moves into the cache
pub fn cache_moves(connection: &Connection, moves: &Vec<PokeMove>, species_id: i32) -> Result<()> {
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
        Err(err) => Err(err).into_diagnostic(), // Pass error up
    }
}

/// For a given species and move, retrieve all the learning methods for that move from the cache
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
                    while let Some(row) = moves_sql.next().into_diagnostic()? {
                        // idx corresponds to the order in which columns are declared in table creation statement
                        moves.insert(MoveLearnMethod {
                            method: LearnMethod::from_i32(row.get(3).into_diagnostic()?).unwrap(),
                            level_learned_at: row.get(4).into_diagnostic()?,
                            generation: Generation::from_i32(row.get(5).into_diagnostic()?)
                                .unwrap(),
                        });
                    }
                    Ok(moves)
                }
                Err(e) => Err(e).into_diagnostic(), // Pass the error up
            }
        }
        Err(e) => Err(e).into_diagnostic(), // Pass the error up
    }
}

/// For a given species and vector of abilities, insert each ability into the cache
pub fn cache_abilities(conn: &Connection, abilities: &Vec<String>, species_id: i32) -> Result<()> {
    let mut buffer: Vec<String> = vec![String::from("BEGIN;")];

    for ability in abilities {
        buffer.push(format!(
            "INSERT INTO abilities (name, species_id) VALUES ('{ability}', '{species_id}');",
        ))
    }

    buffer.push(String::from("COMMIT;"));

    let res = conn.execute_batch(buffer.join(" ").as_str());
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err).into_diagnostic(), // Pass error up
    }
}

pub fn fetch_abilities(conn: &Connection, species_id: i32) -> Result<HashSet<String>> {
    let stmt = conn.prepare("SELECT * FROM abilities WHERE species_id = ?1;");

    match stmt {
        Ok(mut res) => {
            let mut abilities: HashSet<String> = HashSet::new();
            let ability_sql = res.query(rusqlite::params![species_id]);

            match ability_sql {
                Ok(mut ability_sql) => {
                    // Must use weird next() interface as Rows object does not implement Iterator trait
                    while let Some(row) = ability_sql.next().into_diagnostic()? {
                        // idx corresponds to the order in which columns are declared in table creation statement
                        abilities.insert(row.get(1).into_diagnostic()?);
                    }
                    Ok(abilities)
                }

                Err(e) => Err(e).into_diagnostic(),
            }
        }

        Err(e) => Err(e).into_diagnostic(),
    }
}

/// For each type of pokeball known to PokeAPI, cache them.
pub fn cache_balls(conn: &Connection, balls: HashSet<String>) -> Result<()> {
    let mut buffer: Vec<String> = vec![String::from("BEGIN;")];

    for ball in balls {
        buffer.push(format!("INSERT INTO balls (name) VALUES ('{ball}');",))
    }

    buffer.push(String::from("COMMIT;"));

    let res = conn.execute_batch(buffer.join(" ").as_str());
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err).into_diagnostic(), // Pass error up
    }
}

/// Retrieve a set of each type of pokeball from the cache
pub fn fetch_balls(conn: &Connection) -> Result<HashSet<String>> {
    let stmt = conn.prepare("SELECT * FROM balls");

    match stmt {
        Ok(mut res) => {
            let mut balls: HashSet<String> = HashSet::new();
            let mut balls_sql = res.query(rusqlite::params![]);

            match balls_sql {
                Ok(mut balls_sql) => {
                    while let Some(row) = balls_sql.next().into_diagnostic()? {
                        balls.insert(row.get(1).into_diagnostic()?);
                    }
                    Ok(balls)
                }
                Err(e) => Err(e).into_diagnostic(),
            }
        }
        Err(e) => Err(e).into_diagnostic(),
    }
}

/// A convenience function to cache a species and all of its related fields all at once.
pub fn cache_entire_pokemon(
    conn: &Connection,
    species: &str,
    poke_moves: &Vec<PokeMove>,
    abilities: &Vec<String>,
) -> Result<i32, Error> {
    cache_species(conn, species)?;
    let species_id = fetch_species_id(conn, species)?;
    cache_moves(conn, poke_moves, species_id)?;
    cache_abilities(conn, abilities, species_id)?;
    Ok(species_id)
}

/// A convenience function that pulls data from PokeAPI and then caches the results
pub fn get_and_cache_pokemon(species: &str) -> Result<i32, Error> {
    info(format!("Fetching {species}'s info. This will only happen once!").as_str());
    let conn = get_db_connection();
    let pokemon_json = api_get_pokemon(species);
    info("Fetching moves...");
    let pokemon_moves = api_get_pokemon_moves(&pokemon_json);
    info("Fetching abilities...");
    let pokemon_abilities = api_get_pokemon_abilities(&pokemon_json);
    info("Caching results...");
    cache_entire_pokemon(&conn, &species, &pokemon_moves, &pokemon_abilities)
}
