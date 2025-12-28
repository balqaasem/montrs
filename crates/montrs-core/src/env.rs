use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EnvError {
    MissingKey(String),
    InvalidType(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::MissingKey(k) => write!(f, "Missing environment variable: {}", k),
            EnvError::InvalidType(k) => write!(f, "Invalid type for environment variable: {}", k),
        }
    }
}

impl Error for EnvError {}

pub trait FromEnv: Sized {
    fn from_env(val: String) -> Result<Self, EnvError>;
}

impl FromEnv for String {
    fn from_env(val: String) -> Result<Self, EnvError> {
        Ok(val)
    }
}

pub trait EnvConfig: Send + Sync + 'static {
    fn get_var(&self, key: &str) -> Result<String, EnvError>;
}

pub trait EnvConfigExt: EnvConfig {
    fn get<T: FromEnv>(&self, key: &str) -> Result<T, EnvError> {
        self.get_var(key).and_then(T::from_env)
    }
}

impl<T: EnvConfig + ?Sized> EnvConfigExt for T {}

pub struct TypedEnv {
    // Standard implementation using std::env
}

impl EnvConfig for TypedEnv {
    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        std::env::var(key).map_err(|_| EnvError::MissingKey(key.to_string()))
    }
}
