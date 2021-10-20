use std::fmt;
#[derive(Debug, Clone)]
pub struct ValueError {
    pub message: String,
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValueError {}
