use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;

pub struct Storage {
    filename: String,
}

impl Storage {
    pub fn new(filename: &str) -> Storage {
        Storage {
            filename: filename.to_string(),
        }
    }

    pub fn put(&self, data: &Vec<u8>) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;
        file.write_all(data)?;
        Ok(())
    }

    pub fn get(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut f = File::open(&self.filename)?;
        let metadata = fs::metadata(&self.filename)?;
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        Ok(buffer)
    }
}
