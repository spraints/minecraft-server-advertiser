use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

const BROADCAST_ADDR: &'static str = "224.0.2.60";
const LISTEN_ADDR: &'static str = "0.0.0.0:4445";

fn main() {
    let bcast = Ipv4Addr::from_str(BROADCAST_ADDR).unwrap();
    let socket = UdpSocket::bind(LISTEN_ADDR).unwrap();
    socket
        .join_multicast_v4(&bcast, &Ipv4Addr::UNSPECIFIED)
        .unwrap();
    println!("Listening on {LISTEN_ADDR}...");
    loop {
        let mut buf = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        let b = &buf[..amt];
        match str::from_utf8(b) {
            Ok(s) => println!("From {src}: len={amt} {s:?}"),
            Err(_) => println!("From {src}: len={amt}: {b:?}"),
        };
    }
}
