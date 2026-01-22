// Standard Error Type for the program

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