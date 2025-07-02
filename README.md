# PokeSpecRS
CLI Utility to Generate Pokemon Specifications from the Command Line

# Features

**Fast**: Written in Rust, PokeSpecRS gets the job done quickly. It utilizes an optional local cache to further enhance responsiveness.  _(Coming Soon!)_

**Tested**: PokeSpecRS features a large and robust collection of tests that are run to ensure your spec gets generated without error, every time. _(Coming Soon!)_

**Readable**: With an emphasis on clarity, PokeSpecRS ensures that its outputs are legible and concise.

**Correct**: PokeSpecRS ensures your specs are legal, to the best of its abilities. Leveraging PokeAPI, PokeSpecRS does the hard work for you.

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
- `ball` (optional): The ball that the pokemon was caught in. Default: `Pokeball`
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
- `gen` (optional): Coerce the moveset, ball, and abilities to be valid for a particular generation. Default: Assumed highest-legal generation

# Aknowlegments

- [Rust-Cli Guide](https://rust-cli.github.io/book/index.html)
- [CLAP](https://docs.rs/clap/latest/clap/index.html)
- [PokeAPI](https://pokeapi.co/)
- Reqwest
- [Turso Database, SQLite](https://github.com/tursodatabase/turso)