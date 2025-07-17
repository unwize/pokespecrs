use crate::enums::{Generation, LearnMethod};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PokeMove {
    pub name: String,
    pub generations: HashSet<MoveLearnMethod>,
}

impl Display for PokeMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} |{}|", self.name, self.generations.iter().format(","))
    }
}
