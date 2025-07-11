use crate::spec::PokeSpec;
use std::error::Error;
use std::fmt::{Display, Formatter};

// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub enum SpecFailure {
    Move(String), // Name of the move
    Level,
    Shiny,
    Ball,
    Iv(String, u16),     // Name of stat, value of stat
    Ev(String, u16),     // Name of stat, value of stat
    EvMax(u16),          // Sum total of values
    InvalidStat(String), // Name of stat
}

#[derive(Debug, Clone)]
pub struct SpecError {
    pub kind: SpecFailure,
    pub message: Option<String>,
    pub context: Option<PokeSpec>,
}

impl SpecError {
    pub fn new(kind: SpecFailure, message: Option<String>, context: Option<PokeSpec>) -> SpecError {
        Self {
            kind,
            message,
            context,
        }
    }
}

pub fn explain_spec_error(err: &SpecError) -> String {
    let ctx = err.context.clone();
    match err.clone().kind {
        SpecFailure::Move(mv) => {
            format!(
                "{} is not a valid move for {} lvl {}",
                mv,
                ctx.as_ref().unwrap().species,
                ctx.as_ref().unwrap().species
            )
        }
        SpecFailure::Level => {
            format!(
                "{} is not a valid level for {}",
                ctx.as_ref().unwrap().level,
                ctx.as_ref().unwrap().species
            )
        }
        SpecFailure::Shiny => {
            format!(
                "{} cannot be obtained as a shiny!",
                ctx.as_ref().unwrap().species
            )
        }
        SpecFailure::Ball => {
            format!(
                "{} cannot be obtained in a {}",
                ctx.as_ref().unwrap().species,
                ctx.as_ref().unwrap().ball
            )
        }
        SpecFailure::Iv(name, value) => {
            format!("IV {name} must be between 1 and 31: {}", value)
        }
        SpecFailure::Ev(name, value) => {
            format!("EV {name} must be between 1 and 510: {}", value)
        }
        SpecFailure::EvMax(value) => format!("EVs must have a sum total of less than 510: {value}"),
        SpecFailure::InvalidStat(name) => {
            format!("{} is not a valid stat", name)
        }
    }
}

impl Display for SpecError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", explain_spec_error(self))
    }
}

impl Error for SpecError {}
