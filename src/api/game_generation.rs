use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::collections::HashMap;
use itertools::Itertools;
use crate::console::info;

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

impl Generation {
    pub fn to_string(&self) -> String {
        match self {
            Generation::GEN1 => "Gen 1".to_string(),
            Generation::GEN2 => "Gen 2".to_string(),
            Generation::GEN3 => "Gen 3".to_string(),
            Generation::GEN4 => "Gen 4".to_string(),
            Generation::GEN5 => "Gen 5".to_string(),
            Generation::GEN6 => "Gen 6".to_string(),
            Generation::GEN7 => "Gen 7".to_string(),
            Generation::GEN8 => "Gen 8".to_string(),
            Generation::GEN9 => "Gen 9".to_string(),
        }
    }
}

lazy_static! {
    static ref GENERATION_MAP: HashMap<String, Generation> = HashMap::from([
        ("red-blue".to_string(), Generation::GEN1),
        ("yellow".to_string(), Generation::GEN1),
        ("gold-silver".to_string(), Generation::GEN2),
        ("crystal".to_string(), Generation::GEN2),
        ("ruby-sapphire".to_string(), Generation::GEN3),
        ("emerald".to_string(), Generation::GEN3),
        ("firered-leafgreen".to_string(), Generation::GEN3),
        ("diamond-pearl".to_string(), Generation::GEN4),
        ("platinum".to_string(), Generation::GEN4),
        ("heartgold-soulsilver".to_string(), Generation::GEN4),
        ("black-white".to_string(), Generation::GEN5),
        ("black-2-white-2".to_string(), Generation::GEN5),
        ("x-y".to_string(), Generation::GEN6),
        ("omega-ruby-alpha-sapphire".to_string(), Generation::GEN6),
        ("sun-moon".to_string(), Generation::GEN7),
        ("ultra-sun-ultra-moon".to_string(), Generation::GEN7),
        ("sword-shield".to_string(), Generation::GEN8),
        ("brilliant-diamond-and-shining-pearl".to_string(), Generation::GEN4),
        ("scarlet-violet".to_string(), Generation::GEN9),
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
    println!("{}", GENERATION_MAP.keys().format(","));
    // Check if the str can be parsed into an int. If so, pull the generation value from the enum directly.
    let numeric_gen = val.parse::<u8>();
    if numeric_gen.is_ok() {
        return Some(Generation::from_u8(numeric_gen.unwrap()).unwrap());
    }

    // If the str is not an int, check if it is any of the version markers from PokeAPI and return the corresponding generation
    if GENERATION_MAP.contains_key(val) {
        Some(GENERATION_MAP[val].clone())
    } else {
        info(format!("Failed to get generation for {val}").as_str());
        None
    }
}
