use core::panic;
use std::{
    env,
    net::{SocketAddr, UdpSocket},
};

use regex::Regex;

struct Wol {
    packet: Vec<u8>,
}

impl Wol {
    pub fn from_bytes(mac_addr: Vec<u8>) -> Wol {
        Wol {
            packet: Wol::create_packet(mac_addr),
        }
    }
    pub fn from_string(mac_address: &str) -> Wol {

        let mac_addr_bytes: Vec<u8> = mac_address
            .split(":")
            .map(|x| u8::from_str_radix(x, 16).unwrap())
            .collect();
        Wol::from_bytes(mac_addr_bytes)
    }
    fn create_packet(mac_addr: Vec<u8>) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::new();
        packet.extend([0xff; 6]);
        for _ in 0..16 {
            packet.extend(mac_addr.iter());
        }
        packet
    }
    pub fn send_magic_packet(&self) -> std::io::Result<()> {
        let src: SocketAddr = "0.0.0.0:0".parse().unwrap();
        let dst: SocketAddr = "255.255.255.255:9".parse().unwrap();
        let socket = UdpSocket::bind(src)?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.packet, dst)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mac_address = &args[1];
    let re = Regex::new(
        r"^[0-9A-F]{2}:[0-9A-F]{2}:[0-9A-F]{2}:[0-9A-F]{2}:[0-9A-F]{2}:[0-9A-F]{2}$",
    )
    .unwrap();
    if !re.is_match(mac_address) {
        println!("Invalid MAC address: {}", mac_address);
        panic!();
    }
    let wol = Wol::from_string(mac_address);
    wol.send_magic_packet()?;
    println!("done");
    Ok(())
}
