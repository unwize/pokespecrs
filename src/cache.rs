use rusqlite::{Connection, Result};
use std::error::Error;

const CACHE_PATH: &str = "./cache.db3";

pub fn get_db_connection() -> Connection {
    Connection::open(CACHE_PATH).expect("Failed to connect to cache!")
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
                FOREIGN KEY(species) NOT NULL REFERENCES pokemon(id),\
                method INTEGER NOT NULL,
                level_learned_at INTEGER,
                generation INTEGER NOT NULL,
        )",
        ())?;

    Ok(())
}

pub fn insert_pokemon(connection: &Connection, species: &str) -> Result<()> {

    connection.execute("INSERT INTO pokemon (species) VALUES (?1)", ())?;

    Ok(())
}
