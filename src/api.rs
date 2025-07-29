use crate::api::pokemon_move::{MoveLearnMethod, PokeMove};
use crate::console::err;
use crate::enums::{Generation, LearnMethod};
use std::collections::HashSet;
use std::process::exit;

pub mod pokemon_move;

pub fn get_poke_api_route(route: &str) -> serde_json::Value {
    static BASE_URI: &str = "https://pokeapi.co/api/v2/";

    let req = reqwest::blocking::get(String::from(BASE_URI) + route);
    if req.is_err() {
        err("Failed to communicate with PokeAPI. Do you have an internet connection?");
        exit(-1);
    }
    let json = req.unwrap().json();
    if json.is_err() {
        err((String::from(
            "Failed to parse JSON from PokeAPI's response. Is the endpoint correct? ",
        ) + BASE_URI + route)
            .as_str());
        exit(-1);
    }
    let json: serde_json::Value = json.unwrap();
    json
}

pub fn api_get_pokemon(species: &str) -> serde_json::Value {
    get_poke_api_route((String::from("pokemon/") + species).as_str())
}

/// For a given Pokemon JSON object, extract a structured list of moves.
pub fn api_get_pokemon_moves(pokemon_json: &serde_json::Value) -> Vec<PokeMove> {
    // The returned list of Move structs
    let mut moves: Vec<PokeMove> = Vec::new();

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
                generation: Generation::parse(
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

        moves.push(PokeMove {
            name: String::from(pokemon_move["move"]["name"].as_str().unwrap()),
            generations: methods,
        });
    }

    moves
}

pub fn api_get_pokemon_abilities(pokemon_json: &serde_json::Value) -> Vec<String> {
    let mut abilities: Vec<String> = Vec::new();

    for ability in pokemon_json["abilities"].as_array().unwrap() {
        abilities.push(String::from(ability["ability"]["name"].as_str().unwrap()));
    }

    abilities
}

pub fn api_get_balls() -> HashSet<String> {
    static BALL_URI: &str = "item-pocket/3/";
    let response = get_poke_api_route(BALL_URI);

    let mut balls: HashSet<String> = HashSet::new();
    for subroute in response.as_array().unwrap() {
        let subroute_json = get_poke_api_route(subroute["url"].as_str().unwrap());
        for ball in subroute_json["items"].as_array().unwrap() {
            balls.insert(ball["name"].as_str().unwrap().to_string());
        }
    }
    balls
}
