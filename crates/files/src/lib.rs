use std::{fs, io::Write};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The errors that could happen when loading a TOML file.
#[derive(Debug, Error)]
pub enum Error {
    /// Failed to interact with the file system
    #[error("Failed to interact with the file system due to {0}")]
    IO (std::io::Error),

    /// Failed to serialize the provided struct to TOML
    #[error("Failed to serialize the provided struct to TOML due to {0}")]
    Serialize (toml::ser::Error),

    /// Failed to deserialize the TOML into the requested struct
    #[error("Failed to deserialize the TOML into the requested struct due to {0}")]
    Deserialize (toml::de::Error),
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}
impl From<toml::ser::Error> for Error {
    fn from(value: toml::ser::Error) -> Self {
        Error::Serialize(value)
    }
}
impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::Deserialize(value)
    }
}

// #===========================#
// #=== TOML IMPLEMENTATION ===#

/// Unit struct holding methods for interacting with TOML files.
pub struct Toml;
impl Toml {
    /// Tries to load a TOML file from path. If it doesn't find one, it creates one from default.
    pub fn get<T:for<'de> Deserialize<'de> + Serialize + Default>(file_path: &str) -> Result<T, Error> {
        // Create the config if it does not exist
        if !fs::exists(file_path)? {
            Self::create_default::<T>(file_path)?;
        }

        // Try to load the config file
        Self::load::<T>(file_path)
    }
    /// Tries to create a new TOML file from the struct provided.
    pub fn create<T:Serialize>(file_path: &str, content: &T) -> Result<(), Error> {
        // Create new file or return with error
        let mut file = fs::File::create(file_path)?;

        // Serialize the struct to TOML string
        let parsed = toml::to_string(content)?;

        // Write the TOML string to the file
        Ok(file.write_all(parsed.as_bytes())?)
    }
    /// Tries to create a new TOML file from struct default.
    pub fn create_default<T:Default + Serialize>(file_path: &str) -> Result<(), Error> {
        // Create new file or return with error
        let mut file = fs::File::create(file_path)?;

        // Serialize the struct to TOML string
        let parsed = toml::to_string(&T::default())?;

        // Write the TOML string to the file
        Ok(file.write_all(parsed.as_bytes())?)
    }
    /// Tries to save the struct to a TOML file.
    pub fn save<T:Serialize>(file_path: &str, content: &T) -> Result<(), Error> {
        // Open the file or return with error
        let mut file = fs::OpenOptions::new().write(true).open(file_path)?;

        // Serialize the struct to TOML string
        let parsed = toml::to_string(content)?;

        // Write the TOML string to the file
        Ok(file.write_all(parsed.as_bytes())?)
    }
    /// Tries to load a TOML file into the required struct.
    pub fn load<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<T, Error> {
        // Load the file to string or return with error
        let content = fs::read_to_string(file_path)?;

        // Deserialize the toml config into the struct
        Ok(toml::from_str::<T>(&content)?)
    }
}
