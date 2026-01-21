use crate::chamsys::midi_through_to_chamsys;
use crate::test::{dummy_midi_out};

mod midi_io;
mod midi_translator;
mod notes_to_stops;
mod test;
mod chamsys;

fn main() {
    // Currently just running the dummy program for testing
    match midi_through_to_chamsys() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };
}

pub struct ProgramError {
    message: String,
}

impl ProgramError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Macro for conveniently returning a ProgramError
#[macro_export]
macro_rules! return_err {
    ($e:expr) => {
        return Err(ProgramError::new($e.into()))
    };
}


