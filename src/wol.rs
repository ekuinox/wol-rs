use std::net::{UdpSocket, SocketAddr};
use mac_address::MacAddress;
use anyhow::Result;

/// MACの長さ
const MAC_LENGTH: usize = 6;

/// MACを繰り返す回数
const MAC_REPEAT_TIMES: usize = 16;

/// マジックパケットの先頭に付与するパケット
const HEAD: [u8; 6] = [0xFF; 6];

/// マジックパケットの長さ
const PACKET_SIZE: usize = MAC_LENGTH * MAC_REPEAT_TIMES + HEAD.len();

/// 送信時にバインドするアドレス
const BIND_TO: &'static str = "0.0.0.0:0";

/// 送信先ポート
const SEND_TO_PORT: u8 = 9;

/// MACからパケットを作成する
fn create_wol_packet(mac_str: &str) -> Result<Vec<u8>> {
    let address = mac_str.parse::<MacAddress>()?.bytes();
    let mut buffer: Vec<u8> = Vec::with_capacity(PACKET_SIZE);
    buffer.extend(HEAD.iter().copied());
    for _i in 0..MAC_REPEAT_TIMES {
        buffer.extend(address.iter().copied());
    }
    Ok(buffer)
}

/// 対象のMACに対してパケットを送信する
pub fn wol(mac_str: &str, to_addr: &str) -> Result<usize> {
    let packet = create_wol_packet(mac_str)?;
    
    let socket = UdpSocket::bind(BIND_TO)?;
    socket.set_broadcast(true)?;

    let to_addr = format!("{:}:{:}", to_addr, SEND_TO_PORT).parse::<SocketAddr>()?;

    let len = socket.send_to(packet.as_slice(), to_addr)?;

    Ok(len)
}

#[test]
fn test_create_wol_packet() {
    assert_eq!(
        create_wol_packet("10:10:10:10:10:10").ok(),
        Some(vec![
            0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8, 0xffu8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
            0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8, 0x10u8,
        ])
    );
}
