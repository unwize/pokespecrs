use crate::api::game_generation::Generation;
use crate::console::err;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, ToPrimitive, FromPrimitive)]
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

#[derive(Debug, Clone)]
pub struct MoveLearnMethod {
    pub method: LearnMethod,
    pub level_learned_at: Option<u8>,
    pub generation: Generation,
}

impl Display for MoveLearnMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @lvl: {} ({})",
            self.generation.to_string(),
            self.level_learned_at.unwrap_or(0),
            self.method.to_string()
        )
    }
}

#[derive(Debug)]
pub struct Move {
    pub name: String,
    pub generations: Vec<MoveLearnMethod>,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} |{}|", self.name, self.generations.iter().format(","))
    }
}
