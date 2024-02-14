mod config;
mod wol;

use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

use crate::{
    config::{Config, HostConfig},
    wol::wol,
};

#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommand,

    #[clap(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    Up {
        target: String,

        #[clap(short, long)]
        ip: bool,

        #[clap(short, long)]
        mac: bool,
    },
    Add {
        #[clap(short, long, alias = "mac")]
        mac_address: String,

        #[clap(short, long)]
        ip: IpAddr,

        nickname: Option<String>,
    },
    Dump,
}

fn main() -> Result<()> {
    let Cli { subcommand, config } = Cli::parse();
    let path = config.unwrap_or_else(|| Config::path().into());

    if !path.exists() {
        let config = Config::default();
        let toml_text = toml::to_string(&config).expect("Failed to serialize config.");
        std::fs::write(&path, toml_text).expect("Failed to write default config.");
        println!("Config file has been initialized.");
    }

    let mut config = Config::load(&path).expect("Failed to open config.");

    match subcommand {
        Subcommand::Up { target, ip, mac } => {
            let host = match (ip, mac) {
                (true, _) => config
                    .get_host_by_ip(target.parse().unwrap())
                    .expect("Failed to get host.")
                    .clone(),
                (_, true) => HostConfig {
                    ip: Ipv4Addr::UNSPECIFIED.into(),
                    nickname: None,
                    mac: target,
                },
                _ => config
                    .get_host_by_nickname(&target)
                    .expect("Failed to get host.")
                    .clone(),
            };
            let _ = wol(&host.mac, "255.255.255.255").expect("Failed to send wol packet.");
            println!("Packet is sent to {}.", host.nickname.unwrap_or(host.mac));
        }
        Subcommand::Add {
            mac_address,
            ip,
            nickname,
        } => {
            config.add_hosts(vec![HostConfig {
                mac: mac_address,
                nickname,
                ip,
            }]);
            config.dedup_hosts();
            config.save(&path).expect("Failed to write config.");
        }
        Subcommand::Dump => {
            println!("File path: {}", path.to_string_lossy());
            for host in &config.hosts {
                if let Some(nickname) = &host.nickname {
                    println!("{}: {}, {}", nickname, host.ip, host.mac);
                }
            }
        }
    }

    Ok(())
}
