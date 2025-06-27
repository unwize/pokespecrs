# pokespecrs
CLI Utility to Generate Pokemon from a Spec

# Commands

## `generate`

Generate a pokemon.

### args:

- `species` (required): The species of the Pokemon
- `ability` (optional): The ability of the pokemon. Default: random
- `level`, `l` (optional): The level of the Pokemon. Default: `1`
- `nickname`, `n` (optional): The nickname of the Pokemon
- `shiny`, `s` (optional): Is the Pokemon shiny? Default: `False`
- `ot` (optional): The name  of the original trainer. Default: PokeSpecRS
- `tid` (optional): The id of the original trainer. Default: Random
- `sid` (optional): The id of the pokemon. Default: Random
- `gender` (optional): The gender of the pokemon. Default: Random
- `ball` (optional): The ball that the pokemon was caught in. Default: `pokeball`
- `nature` (optional): The nature of the pokemon. Default: Random
- `ivattack` (optional): The IV of the pokemon's attack stat. Default: Random
- `ivspattack` (optional): The IV of the pokemon's special attack stat. Default: Random
- `ivdefense` (optional): The IV of the pokemon's defense stat. Default: Random
- `ivspdefense` (optional): The IV of the pokemon's special defense stat. Default: Random
- `ivspeed` (optional): The IV of the pokemon's speed stat. Default: Random
- `ivhp` (optional): The IV of the pokemon's hp stat. Default: Random
- `evattack` (optional): The EV of the pokemon's attack stat. Default: Random
- `evspattack` (optional): The EV of the pokemon's special attack stat. Default: Random
- `evdefense` (optional): The EV of the pokemon's defense stat. Default: Random
- `evspdefense` (optional): The EV of the pokemon's special defense stat. Default: Random
- `evspeed` (optional): The EV of the pokemon's speed stat. Default: Random
- `evhp` (optional): The EV of the pokemon's hp stat. Default: Random

# Aknowlegments

- [Rust-Cli Guide](https://rust-cli.github.io/book/index.html)
- [CLAP](https://docs.rs/clap/latest/clap/index.html)