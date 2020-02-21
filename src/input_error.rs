use std::fmt;
use std::error;

#[derive(Debug)]
pub enum InputError {
    WrongFormat,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InputError::WrongFormat => f.write_str("Wrong input format"),
        }
    }
}
impl error::Error for InputError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
