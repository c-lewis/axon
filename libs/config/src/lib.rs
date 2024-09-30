use std::{error::Error, fs::File, io::BufReader, path::Path};

use serde::de::DeserializeOwned;

pub mod feature;
pub mod metadata;

pub fn read_config<R: DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<R, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
