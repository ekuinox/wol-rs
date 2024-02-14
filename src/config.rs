use std::{
    env,
    fs::{read_to_string, OpenOptions},
    io::prelude::*,
    net::IpAddr,
    path::Path,
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Host, associates ip to mac.
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct HostConfig {
    pub nickname: Option<String>,
    pub ip: IpAddr,
    pub mac: String,
}

/// configurations for wol-rs
#[derive(Deserialize, Serialize, Default, PartialEq, Clone, Debug)]
pub struct Config {
    pub hosts: Vec<HostConfig>,
}

impl Config {
    /// load configurations from specified path
    pub fn load(path: impl AsRef<Path>) -> Result<Config> {
        let content = read_to_string(path)?;
        let conf = toml::from_str(content.as_str())?;
        Ok(conf)
    }

    /// save configurations from specified path
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string(self)?;
        let mut file = OpenOptions::new().create(true).write(true).open(path)?;
        let r = file.write_all(content.as_bytes())?;
        Ok(r)
    }

    /// return path for linux os
    #[cfg(target_os = "linux")]
    pub fn path() -> String {
        format!(
            "{}/.wol-rs.toml",
            env::var("HOME").expect("$HOME is not defined")
        )
    }

    /// return path for windows os
    #[cfg(target_os = "windows")]
    pub fn path() -> String {
        format!(
            "{}/.wol-rs.toml",
            env::var("USERPROFILE").expect("%USERPROFILE% is not defined")
        )
    }

    #[cfg(target_os = "macos")]
    pub fn path() -> String {
        format!(
            "{}/.wol-rs.toml",
            env::var("HOME").expect("$HOME is not defined")
        )
    }

    /// get hosts by ip address
    /// if host doesn't exist, return `HostNotFound`.
    pub fn get_host_by_ip(&self, ip: IpAddr) -> Result<&HostConfig> {
        let host = self.hosts.iter().find(|host| host.ip == ip);
        if let Some(host) = host {
            Ok(host)
        } else {
            Err(anyhow!("Host is not found."))
        }
    }

    /// get hosts by nickname
    /// if host doesn't exist, return `HostNotFound`.
    pub fn get_host_by_nickname(&self, nickname: &str) -> Result<&HostConfig> {
        let host = self.hosts.iter().find(|host| {
            host.nickname
                .as_ref()
                .map(|t| t.as_str() == nickname)
                .unwrap_or_default()
        });
        if let Some(host) = host {
            Ok(host)
        } else {
            Err(anyhow!("Host is not found."))
        }
    }

    /// add new host
    pub fn add_hosts(&mut self, hosts: Vec<HostConfig>) {
        self.hosts.extend(hosts);
    }

    /// dedup hosts
    pub fn dedup_hosts(&mut self) {
        // remove duplications
        let _ = self.hosts.dedup_by(|a, b| {
            let eq = a == b;
            // a will be removed, clone b's nickname to a
            if eq && b.nickname.is_none() {
                b.nickname = a.nickname.clone();
            }
            eq
        });
    }
}
