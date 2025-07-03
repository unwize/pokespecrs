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

    pub fn from(value: &str) -> Option<Self> {
        match value {
            "red-blue" => Some(Generation::GEN1),
            "yellow" => Some(Generation::GEN1),
            "gold-silver" => Some(Generation::GEN2),
            "crystal" => Some(Generation::GEN2),
            "ruby-sapphire" => Some(Generation::GEN3),
            "emerald" => Some(Generation::GEN3),
            "firered-leafgreen" => Some(Generation::GEN3),
            "xd" => Some(Generation::GEN3),
            "colosseum" => Some(Generation::GEN3),
            "diamond-pearl" => Some(Generation::GEN4),
            "platinum" => Some(Generation::GEN4),
            "heartgold-soulsilver" => Some(Generation::GEN4),
            "black-white" => Some(Generation::GEN5),
            "black-2-white-2" => Some(Generation::GEN5),
            "x-y" => Some(Generation::GEN6),
            "omega-ruby-alpha-sapphire" => Some(Generation::GEN6),
            "sun-moon" => Some(Generation::GEN7),
            "ultra-sun-ultra-moon" => Some(Generation::GEN7),
            "sword-shield" => Some(Generation::GEN8),
            "legends-arceus" => Some(Generation::GEN8),
            "brilliant-diamond-and-shining-pearl" => Some(Generation::GEN4),
            "scarlet-violet" => Some(Generation::GEN9),
            "1" => Some(Generation::GEN1),
            "2" => Some(Generation::GEN2),
            "3" => Some(Generation::GEN3),
            "4" => Some(Generation::GEN4),
            "5" => Some(Generation::GEN5),
            "6" => Some(Generation::GEN6),
            "7" => Some(Generation::GEN7),
            "8" => Some(Generation::GEN8),
            "9" => Some(Generation::GEN9),
            _ => None,
        }
    }
}
