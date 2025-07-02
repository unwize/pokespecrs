use crate::console::{err, info};
use std::collections::HashMap;
use std::process::exit;

mod game_generation;

pub fn get_pokemon(species: &str) {
    let req = reqwest::blocking::get(format!("https://pokeapi.co/api/v2/pokemon/{species}"));
    if req.is_err() {
        err("Failed to communicate with PokeAPI. Do you have an internet connection?");
        exit(-1);
    }
    let json = req.unwrap().json();
    if json.is_err() {
        err("Failed to parse JSON from PokeAPI's response. Is PokeAPI available?");
        exit(-1);
    }
    let json: serde_json::Value = json.unwrap();
    info(format!("Pokemon: {json}").as_str());
}
