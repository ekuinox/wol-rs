use std::net::{UdpSocket, SocketAddr};
use clap::{App, Arg};
use mac_address::MacAddress;
use anyhow::Result;

const CHUNK_LENGTH: usize = 6;
const MAC_REPEAT_TIMES: usize = 16;
const HEAD: [u8; 6] = [0xFF; 6];
const PACKET_SIZE: usize = CHUNK_LENGTH * MAC_REPEAT_TIMES + HEAD.len();
const BIND_TO: &'static str = "0.0.0.0:0";
const SEND_TO_PORT: u8 = 9;

fn send_wol_packet(mac_str: &str, to_addr: &str) -> Result<usize> {
    let address = mac_str.parse::<MacAddress>()?.bytes();
    
    let mut buffer: Vec<u8> = Vec::with_capacity(PACKET_SIZE);
    buffer.extend(HEAD.iter().copied());
    for _i in 0..MAC_REPEAT_TIMES {
        buffer.extend(address.iter().copied());
    }
    
    let socket = UdpSocket::bind(BIND_TO)?;

    socket.set_broadcast(true)?;

    let to_addr = format!("{:}:{:}", to_addr, SEND_TO_PORT).parse::<SocketAddr>()?;

    let len = socket.send_to(
        buffer.as_slice(),
        to_addr
    )?;

    Ok(len)
}

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
        let _ = send_wol_packet(mac, "255.255.255.255");
        return;
    }
}
