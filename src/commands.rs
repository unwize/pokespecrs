use crate::Cli;

/// A trait that defines the interface for executing command logic
trait Command {
    fn execute(&self, args: Cli);
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
    fn execute(&self, args: Cli) {

        // TODO: Get ability or random ability
    }
}
