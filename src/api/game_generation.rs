use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::collections::HashMap;

#[derive(FromPrimitive, ToPrimitive, Clone, Debug)]
pub enum Generation {
    GEN1 = 1,
    GEN2 = 2,
    GEN3 = 3,
    GEN4 = 4,
    GEN5 = 5,
    GEN6 = 6,
    GEN7 = 7,
    GEN8 = 8,
    GEN9 = 9,
}

lazy_static! {
    static ref GENERATION_MAP: HashMap<&'static str, Generation> = HashMap::from([
        ("red-blue", Generation::GEN1),
        ("yellow", Generation::GEN1),
        ("gold-silver", Generation::GEN2),
        ("crystal", Generation::GEN2),
        ("ruby-sapphire", Generation::GEN3),
        ("emerald", Generation::GEN3),
        ("firered-leafgreen", Generation::GEN3),
        ("diamond-pearl", Generation::GEN4),
        ("platinum", Generation::GEN4),
        ("heartgold-soulsilver", Generation::GEN4),
        ("black-white", Generation::GEN5),
        ("black-2-white-2", Generation::GEN5),
        ("x-y", Generation::GEN6),
        ("omega-ruby-alpha-sapphire", Generation::GEN6),
        ("sun-moon", Generation::GEN7),
        ("ultra-sun-ultra-moon", Generation::GEN7),
        ("sword-shield", Generation::GEN8),
        ("brilliant-diamond-and-shining-pearl", Generation::GEN4),
        ("scarlet-violet", Generation::GEN9),
    ]);
}

/// For a given str, parse out the corresponding Pokemon game generation.
///
/// # Examples
///
/// ```
/// assert_eq(get_generation("1", Generation::GEN1)
/// assert_eq(get_generation("platinum", Generation::GEN4)
/// ```
pub fn get_generation(val: &str) -> Option<Generation> {
    // Check if the str can be parsed into an int. If so, pull the generation value from the enum directly.
    let numeric_gen = val.parse::<u8>();
    if numeric_gen.is_ok() {
        return Some(Generation::from_u8(numeric_gen.unwrap()).unwrap());
    }

    // If the str is not an enum, check if it is any of the version markers from PokeAPI and return the corresponding generation
    if GENERATION_MAP.contains_key(val) {
        Some(GENERATION_MAP[val].clone())
    } else {
        None
    }
}
