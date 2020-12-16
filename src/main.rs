mod configurations;
mod wol;

#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};
use anyhow::Result;
use wol::wol;
use configurations::Configurations;

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
        .subcommand(SubCommand::with_name("dump")
            .about("dumps configurations if it existed")
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
    
    if let Some(_) = matches.subcommand_matches("dump") {
        println!("File path: {}", path);
        println!("{:?}", configurations?);
        return Ok(());
    }

    if let Some(mac) = matches.value_of("mac") {
        let _ = wol(mac, "255.255.255.255")?;
        println!("send packet to {:}", mac);
        return Ok(());
    }

    if let Some(ip) = matches.value_of("ip") {
        let conf = configurations?;
        let host = conf.get_hosts_by_ip(ip)?;
        let _ = wol(host.mac.as_str(), "255.255.255.255")?;
        println!("send packet to {:}", ip);
    }

    Ok(())
}
