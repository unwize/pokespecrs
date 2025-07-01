use std::fmt;

#[derive(Debug, Clone)]
struct SpecError;

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid PokeSpec!")
    }
}
