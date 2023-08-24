
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self {
            message,
        }
    }

    pub fn from_pest_error(error: pest::error::Error<crate::lexer::Rule>) -> Self {
        Self {
            message: error.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.message)
    }
}