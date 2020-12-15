use anyhow::Result;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;

/// Host, associates ip to mac.
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    ip: String,
    mac: String,
}

/// configurations for wol-rs
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configurations {
    hosts: Vec<Host>,
}

impl Configurations {
    /// load configurations from specified path
    pub fn load(path: &str) -> Result<Configurations> {
        let content = read_to_string(path)?;
        let conf = toml::from_str(content.as_str())?;
        Ok(conf)
    }
    /// save configurations from specified path
    pub fn save(&self, path: &str) -> Result<()> {
        let content = toml::to_string(self)?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)?;
        let r = file.write_all(content.as_bytes())?;
        Ok(r)
    }
    /// return path for linux os
    #[cfg(target_os = "linux")]
    pub fn path<'a>() -> &'a str {
        // TODO: ~/
        "./.wol-rs.toml"
    }
    /// return path for windows os
    #[cfg(target_os = "windows")]
    pub fn path<'a>() -> &'a str {
        // TODO: %USERPROFILE%
        "./.wol-rs.toml"
    }
}
