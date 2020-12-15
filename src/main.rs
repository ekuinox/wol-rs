mod wol;
mod configurations;

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
        .subcommand(SubCommand::with_name("dump")
            .about("dumps configurations if it existed")
        )
        .get_matches();

    let path = Configurations::path();

    // if file does not exists, create with default configurations. 
    // it doesn't matter if the file existed at this stage.
    let configurations = Configurations::load(path).or_else(|_| -> Result<Configurations> {
        let default = Configurations::default();
        let _ = default.save(path)?;
        Ok(default)
    });
    
    if let Some(_) = matches.subcommand_matches("dump") {
        println!("File path: {}", path);
        println!("{:?}", configurations?);
        return Ok(());
    }

    if let Some(mac) = matches.value_of("mac") {
        let _ = wol(mac, "255.255.255.255");
        return Ok(());
    }

    Ok(())
}
