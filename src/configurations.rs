use anyhow::Result;
use thiserror::Error;
use std::fs::{read_to_string, OpenOptions};
use std::io::prelude::*;
use std::env;

/// Host, associates ip to mac.
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub ip: String,
    pub mac: String,
}

/// configurations for wol-rs
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Configurations {
    pub hosts: Vec<Host>,
}

#[derive(Error, Debug)]
pub enum ConfigurationsError {
    #[error("host not found")]
    HostNotFound,
}
pub use ConfigurationsError::*;

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
    pub fn path() -> String {
        format!("{}/.wol-rs.toml", env::var("HOME").expect("$HOME is not defined"))
    }
    /// return path for windows os
    #[cfg(target_os = "windows")]
    pub fn path() -> String {
        format!("{}/.wol-rs.toml", env::var("USERPROFILE").expect("%USERPROFILE% is not defined"))
    }
    /// get hosts by ip address
    /// if host doesn't exist, return `HostNotFound`.
    pub fn get_hosts_by_ip(&self, ip: &str) -> Result<&Host> {
        let ip = self.hosts
            .iter()
            .find(|host| host.ip == ip)
            .ok_or(HostNotFound)?;
        Ok(ip)
    }
}
