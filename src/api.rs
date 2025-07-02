use crate::api::game_generation::get_generation;
use crate::api::pokemon_move::{LearnMethod, Move, MoveLearnMethod};
use crate::console::err;
use std::process::exit;

mod game_generation;
mod pokemon_move;

pub fn get_pokemon(species: &str) -> serde_json::Value {
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
    //info(format!("Pokemon: {json}").as_str());
    json
}

/// For a given Pokemon JSON object, extract a structured list of moves.
pub fn get_pokemon_moves(pokemon_json: &serde_json::Value) -> Vec<Move> {
    // The returned list of Move structs
    let mut moves: Vec<Move> = Vec::new();

    // Iterate through the list of Move JSON objects
    for pokemon_move in pokemon_json["moves"].as_array().unwrap() {
        // A vector containing the learning methods and game generations for the given move
        let mut methods: Vec<MoveLearnMethod> = Vec::new();

        // Iterate through the Move Learning Methods JSON array
        for method in pokemon_move["version_group_details"].as_array().unwrap() {
            methods.push(MoveLearnMethod {
                method: LearnMethod::from(method["move_learn_method"]["name"].as_str().unwrap())
                    .unwrap(),
                level_learned_at: Some(method["level_learned_at"].as_u64().unwrap() as u8),
                generation: get_generation(&method["version_group"]["name"].to_string()).unwrap(),
            })
        }

        moves.push(Move {
            name: pokemon_move["name"].to_string(),
            generations: methods,
        })
    }

    moves
}
