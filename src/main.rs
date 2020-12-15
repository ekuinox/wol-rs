mod wol;

use clap::{App, Arg};
use wol::wol;

fn main() {
    let matches = App::new("wol")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("mac")
            .help("target mac address <ff:ff:ff:ff:ff:ff>")
            .takes_value(true)
            .short("m")
    ).get_matches();

    if let Some(mac) = matches.value_of("mac") {
        let _ = wol(mac, "255.255.255.255");
        return;
    }
}
