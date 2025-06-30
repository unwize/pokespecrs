use std::collections::HashMap;
use crate::enums::Gender;
use crate::spec::PokeSpec;

/// A trait that defines the interface for executing command logic
trait Command {
    fn execute(&self);
}

struct Generate;

impl Command for Generate {
    /// Generate a PokeSpec from a given set of arguments.
    /// See: README.md for more.
    ///
    /// Required Args / Args with Defaults
    ///  - species
    ///  - level
    ///  - shiny
    ///  - ot
    ///  - ball
    ///
    /// Optional Args with Random Defaults
    /// - tid
    /// - sid
    /// - gender
    /// - nature
    /// - ivs
    /// - evs
    ///
    /// Optional Args with No Default
    /// - nickname
    /// - moveset
    fn execute(&self) {

        // TODO: Get ability or random ability
    }
}