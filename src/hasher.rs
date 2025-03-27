use crate::Arc;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn get_file_hash(file_path: Arc<PathBuf>) -> Result<String, io::Error> {
    let mut file = File::open(file_path.as_path())?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let hash = hasher.finalize();
    let hash_hex = hash
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    Ok(hash_hex)
}
