use crate::api::game_generation::Generation;
use crate::api::pokemon_move::{LearnMethod, Move, MoveLearnMethod};
use crate::console::err;
use std::collections::HashSet;
use std::process::exit;

pub mod game_generation;
pub mod pokemon_move;

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
    json
}

/// For a given Pokemon JSON object, extract a structured list of moves.
pub fn get_pokemon_moves(pokemon_json: &serde_json::Value) -> Vec<Move> {
    // The returned list of Move structs
    let mut moves: Vec<Move> = Vec::new();

    // Iterate through the list of Move JSON objects
    for pokemon_move in pokemon_json["moves"].as_array().unwrap() {
        // A vector containing the learning methods and game generations for the given move
        let mut methods: HashSet<MoveLearnMethod> = HashSet::new();

        // Iterate through the Move Learning Methods JSON array
        for method in pokemon_move["version_group_details"].as_array().unwrap() {
            methods.insert(MoveLearnMethod {
                method: LearnMethod::from(method["move_learn_method"]["name"].as_str().unwrap())
                    .unwrap(),
                level_learned_at: Some(method["level_learned_at"].as_u64().unwrap() as u8),
                generation: Generation::from(
                    &method["version_group"]["name"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                )
                .expect(
                    format!(
                        "Failed to get version group from str: '{}'",
                        method["version_group"]["name"].as_str().unwrap()
                    )
                    .as_str(),
                ),
            });
        }

        moves.push(Move {
            name: String::from(pokemon_move["move"]["name"].as_str().unwrap()),
            generations: methods,
        });
    }

    moves
}
