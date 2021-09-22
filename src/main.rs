mod configurations;
mod wol;

#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};
use anyhow::Result;
use wol::wol;
use configurations::{Configurations, Host};

fn main() -> Result<()> {
    let matches = App::new("wol")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("mac")
            .help("target mac address <ff:ff:ff:ff:ff:ff>")
            .takes_value(true)
            .short("m")
        )
        .arg(Arg::with_name("ip")
            .takes_value(true)
            .short("i")
        )
        .arg(Arg::with_name("nickname")
            .takes_value(true)
            .short("n")
        )
        .subcommand(SubCommand::with_name("dump")
            .about("dumps configurations if it existed")
        )
        .subcommand(SubCommand::with_name("register")
            .about("register new host")
            .arg(Arg::with_name("ip")
                .required(true)
                .long("ip")
                .short("i")
                .takes_value(true)
            )
            .arg(Arg::with_name("mac")
                .required(true)
                .long("mac")
                .short("m")
                .takes_value(true)
            )
            .arg(Arg::with_name("nickname")
               .long("nickname")
               .short("n")
               .takes_value(true)
            )
        )
        .get_matches();

    let path = Configurations::path();

    // if file does not exists, create with default configurations. 
    // it doesn't matter if the file existed at this stage.
    let configurations = Configurations::load(path.as_str()).or_else(|_| -> Result<Configurations> {
        let default = Configurations::default();
        let _ = default.save(path.as_str())?;
        Ok(default)
    });

    // debug dump configuration with file path.
    if let Some(_) = matches.subcommand_matches("dump") {
        let conf = match configurations {
            Ok(conf) => conf,
            _ => { return Ok(()); },
        };
        println!("File path: {}", path);
        for host in &conf.hosts {
            if let Some(nickname) = &host.nickname {
                println!("{}: {}, {}", nickname, host.ip, host.mac);
            }
        }
        return Ok(());
    }

    // register new host with ip, mac, nickname(option).
    if let Some(sub) = matches.subcommand_matches("register") {
        let mut conf = configurations?;
        let ip = sub.value_of("ip").expect("ip is expected").to_string();
        let mac = sub.value_of("mac").expect("mac is expected").to_string();
        let nickname = sub.value_of("nickname").map(|s| s.to_string());
        let host = Host {
            ip, mac, nickname
        };
        conf.add_hosts(vec![host]);
        conf.dedup_hosts();
        conf.save(path.as_str())?;
        return Ok(());
    }

    // wol by mac addres.
    if let Some(mac) = matches.value_of("mac") {
        let _ = wol(mac, "255.255.255.255")?;
        println!("send packet to {:}", mac);
        return Ok(());
    }

    // wol by ip address.
    if let Some(ip) = matches.value_of("ip") {
        let conf = configurations?;
        let host = conf.get_host_by_ip(ip)?;
        let _ = wol(host.mac.as_str(), "255.255.255.255")?;
        println!("send packet to {:}", ip);
        return Ok(());
    }

    // wol by host nickname
    if let Some(nickname) = matches.value_of("nickname").map(|s| s.to_string()) {
        let conf = configurations?;
        let host = conf.get_host_by_nickname(nickname)?;
        let _ = wol(host.mac.as_str(), "255.255.255.255")?;
        return Ok(());
    }

    Ok(())
}
