trait PokeDataSource {
    fn is_species(species: &str) -> bool; // Is the given species valid?
    fn is_moves_valid(species: &str, moves: &Vec<String>) -> Vec<String>;  // For a list of moves, are any of these moves invalid?
    fn is_ability_valid(species: &str, ability: &str) -> bool;
}

fn validate_pokemon(pokemon: PokeSpec) {

}