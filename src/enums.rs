use crate::console::err;
use miette::{miette, Result};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}

/// A basic string-to-enum conversion
impl TryFrom<&str> for Gender {
    type Error = miette::Error;  // Catch-all typing to cover ad-hoc instantiated error via miette macro

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "genderless" => Ok(Gender::Genderless),
            _ => Err(miette!("'{}' is not a valid pokemon gender!", value)),
        }
    }
}

impl Into<String> for Gender {
    fn into(self) -> String {
        match self {
            Gender::Male => "male".to_string(),
            Gender::Female => "female".to_string(),
            Gender::Genderless => "genderless".to_string(),
        }
    }
}



#[derive(FromPrimitive, ToPrimitive, Clone, Debug, Eq, Hash, PartialEq)]
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

impl Into<String> for Generation {
    fn into(self) -> String {
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

impl Generation {

    // Fake `from` function that returns a value, if one matches, from the enum.
    // I cannot decide if its better to do this or implement a TryFrom with increased complexity of custom error types
    pub fn parse(value: &str) -> Option<Self> {
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
            "lets-go-pikachu-lets-go-eevee" => Some(Generation::GEN8),
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

#[derive(Debug, Clone, ToPrimitive, FromPrimitive, Eq, Hash, PartialEq)]
pub enum LearnMethod {
    Machine = 0,
    Egg = 1,
    Tutor = 2,
    LevelUp = 3,
}

impl LearnMethod {
    pub fn from(value: &str) -> Option<Self> {
        match value {
            "machine" => Some(LearnMethod::Machine),
            "egg" => Some(LearnMethod::Egg),
            "tutor" => Some(LearnMethod::Tutor),
            "level-up" => Some(LearnMethod::LevelUp),
            _ => {
                err(format!("Invalid LearnMethod value: '{}'", value).as_str());
                None
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LearnMethod::Machine => "machine".to_string(),
            LearnMethod::Egg => "egg".to_string(),
            LearnMethod::Tutor => "tutor".to_string(),
            LearnMethod::LevelUp => "level-up".to_string(),
        }
    }
}
