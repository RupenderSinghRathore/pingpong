use std::fs;
use std::io;
use std::path::PathBuf;

use super::gameplay::{GamePlay, GameScore};

const CACHE_PATH: &str = ".local/share/pingpong";
const CACHE_FILE: &str = "highest.json";

impl GamePlay {
    pub fn write_cache(&self) -> Result<(), io::Error> {
        let json_data = serde_json::to_string(&self.score)?;
        let cache_dir = match get_cache_path() {
            Some(v) => v,
            None => {
                return Err(io::Error::new(io::ErrorKind::NotFound, "home not found!"));
            }
        };
        let mut p = PathBuf::from(&cache_dir);
        fs::create_dir_all(&p)?;
        p.push(CACHE_FILE);
        fs::write(p, json_data)?;
        Ok(())
    }
    pub fn read_cache(&mut self) -> Result<(), io::Error> {
        let cache_dir = match get_cache_path() {
            Some(v) => v,
            None => {
                return Err(io::Error::new(io::ErrorKind::NotFound, "home not found!"));
            }
        };
        let mut p = PathBuf::from(&cache_dir);
        fs::create_dir_all(&p)?;
        p.push(CACHE_FILE);
        let json_data = fs::read_to_string(p)?;
        let score: GameScore = serde_json::from_str(&json_data)?;
        self.score = score;
        Ok(())
    }
}

fn get_cache_path() -> Option<String> {
    std::env::home_dir().map(|home| format!("{}{}{}", home.to_str().unwrap(), "/", CACHE_PATH))
}
