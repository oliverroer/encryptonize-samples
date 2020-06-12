//! Simple storage of a single file.
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;

/// Represents storage of a single file.
pub struct Storage {
    filename: String,
}

impl Storage {
    /// Create a store containing a single file.
    ///
    /// # Arguments
    /// * `filename` - Name of the stored file.
    pub fn new(filename: &str) -> Storage {
        Storage {
            filename: filename.to_string(),
        }
    }

    /// Create/replace the data in the stored file.
    ///
    /// # Arguments
    /// * `data` - A vector of bytes to store.
    pub fn put(&self, data: &Vec<u8>) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;
        file.write_all(data)?;
        Ok(())
    }

    /// Retrieve any previously stored data.
    pub fn get(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut f = File::open(&self.filename)?;
        let metadata = fs::metadata(&self.filename)?;
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        Ok(buffer)
    }
}
