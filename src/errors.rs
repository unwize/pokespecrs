use miette::Diagnostic;
use std::ops::Add;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub(crate) enum SpecErrors {
    #[error("No such stat: {stat}")]
    #[diagnostic(help(
        "No such stat. Legal stats are Attack, Defense, Special Attack, Special Defense, Speed, and HP"
    ))]
    NoSuchStatError { stat: String },

    #[error("IV value error! {stat}: {value}")]
    #[diagnostic(help("IV values must be between 0 and 31"))]
    IvValueError { stat: String, value: String },

    #[error("EV value error! {stat}: {value}")]
    #[diagnostic(help("EV values must be between 0 and 252"))]
    EvValueError { stat: String, value: String },

    #[error("EV sum error!")]
    #[diagnostic(help("EV sum values must be between 0 and 510"))]
    EvSumError { ev_sum: String },

    #[error("Shiny error: {species}")]
    #[diagnostic(help("This species cannot be obtained as a shiny!"))]
    ShinyError { species: String },

    #[error("Unlearnable move error: {species}: {pk_move}")]
    #[diagnostic(help("This species cannot learn this move!"))]
    UnlearnableMoveError { species: String, pk_move: String },

    #[error(
        "Move level too low! {species}: lvl {level}, {pk_move}. Minimum learn level is {min_level}"
    )]
    #[diagnostic(help("This species cannot learn this move at this level!"))]
    LevelTooLowMoveError {
        species: String,
        pk_move: String,
        level: String,
        min_level: String,
    },

    #[error("Species level too low! {species}: lvl {level}")]
    #[diagnostic(help("This species cannot legally be obtained at such a low level!"))]
    LevelTooLowSpeciesError { species: String, level: String },
}

#[derive(Debug, Diagnostic, Error)]
#[error("Spec error")]
#[diagnostic(help("One or more issues with this spec must be resolved!"))]
pub struct SpecError {
    #[related]
    pub causes: Vec<SpecErrors>,
}

impl Add<SpecError> for SpecError {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SpecError {
            causes: self.causes.into_iter().chain(rhs.causes).collect(),
        }
    }
}
