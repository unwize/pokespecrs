use miette::Diagnostic;
use thiserror::Error;


#[derive(Debug, Error, Diagnostic)]
pub(crate) enum SpecErrors {
    #[error("No such stat: {stat}")]
    #[diagnostic(help("No such stat. Legal stats are Attack, Defense, Special Attack, Special Defense, Speed, and HP"))]
    NoSuchStatError {stat: String},

    #[error("IV value error! {stat}: {value}")]
    #[diagnostic(help("IV values must be between 1 and 31"))]
    IvValueError {stat: String, value: String},

    #[error("EV value error! {stat}: {value}")]
    #[diagnostic(help("EV values must be between 0 and 252"))]
    EvValueError {stat: String, value: String},

    #[error("EV sum error!")]
    #[diagnostic(help("EV sum values must be between 0 and 510"))]
    EvSumError {ev_sum: String},

    #[error("Shiny error: {species}")]
    #[diagnostic(help("This species cannot be obtained as a shiny!"))]
    ShinyError {species: String},

    #[error("Unlearnable move error: {species}: {pk_move}")]
    #[diagnostic(help("This species cannot learn this move!"))]
    UnlearnableMoveError {species: String, pk_move: String},

    #[error("Move level too low! {species}: lvl {level}, {pk_move}")]
    #[diagnostic(help("This species cannot learn this move at this level!"))]
    LevelTooLowMoveError {species: String, pk_move: String, level: String},

    #[error("Species level too low! {species}: lvl {level}")]
    #[diagnostic(help("This species cannot legally be obtained at such a low level!"))]
    LevelTooLowSpeciesError {species: String, level: String}
}

#[derive(Debug, Diagnostic, Error)]
#[error("Spec error")]
#[diagnostic(help("One or more issues with this spec must be resolved!"))]
pub struct SpecError {
    pub related: Vec<SpecErrors>
}


/*
#[derive(Error, Debug, Diagnostic)]
#[error(No Such Stat)]
#[diagnostic(help("No such stat. Legal stats are Attack, Defense, Special Attack, Special Defense, Speed, and HP"))]
pub struct NoSuchStatError {
    #[label("Stat")]
    pub stat: String
}

#[derive(Debug, Diagnostic, Error)]
#[error(IV Value Error)]
#[diagnostic(help("IV values must be between 1 and 31"))]
pub struct IVValueError {
    #[label("IV")]
    pub stat: String,
    #[label("Value")]
    pub value: u16
}

#[derive(Debug, Diagnostic, Error)]
#[error(EV Value Error)]
#[diagnostic(help("EV values must be between 0 and 252"))]
pub struct EVValueError {
    #[label("EV")]
    stat: String,
    #[label("Value")]
    value: u16
}

#[derive(Debug, Diagnostic, Error)]
#[error(EV Sum Error)]
#[diagnostic(help("EV sum values must be between 0 and 510"))]
pub struct EVSumError {
    #[label("EV Sum Total")]
    ev_sum: u16
}

#[derive(Debug, Diagnostic, Error)]
#[error(Shiny Error)]
#[diagnostic(help("This species cannot be obtained as a shiny!"))]
pub struct ShinyError {
    #[label("Species")]
    species: String
}

#[derive(Debug, Diagnostic, Error)]
#[error(Unlearnable Move Error)]
#[diagnostic(help("This species cannot learn this move!"))]
pub struct UnlearnableMoveError {
    #[label("Species")]
    species: String,
    #[label("Move")]
    pk_move: String
}

#[derive(Debug, Diagnostic, Error)]
#[error(Move Level Too Low Error)]
#[diagnostic(help("This species cannot learn this move at this level!"))]
pub struct MoveTooLowError {
    #[label("Species")]
    species: String,
    #[label("Move")]
    pk_move: String,
    #[label("Minimum Level")]
    min_level: u16
}

#[derive(Debug, Diagnostic, Error)]
#[error(Species Level Too Low Error)]
#[diagnostic(help("This species evolves at a level higher than its current level!"))]
pub struct SpeciesLevelTooLowError {
    #[label("Species")]
    species: String,
    #[label("Minimum Level")]
    min_level: u16
}
*/
