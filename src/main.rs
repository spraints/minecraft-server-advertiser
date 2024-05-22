use std::net::UdpSocket;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use gumdrop::Options;

#[derive(Options)]
struct Args {
    #[options(help = "Hosts user's Mincraft handle (required)")]
    user: String,

    #[options(help = "Minecraft server name (required)")]
    name: String,

    #[options(help = "Server port (required)")]
    port: u32,

    #[options(help = "Show this message")]
    help: bool,
}

const BROADCAST_ADDR: &'static str = "224.0.2.60:4445";
const INTERVAL_MILLIS: u64 = 1500;

fn main() {
    let Args {
        user,
        name,
        port,
        help: _,
    } = Args::parse_args_default_or_exit();
    if user == "" || name == "" || port == 0 {
        println!("Usage: mincraft-server-advertiser OPTIONS");
        println!("{}", Args::usage());
        exit(1);
    }

    let msg = format!("[MOTD]{user} - {name}[/MOTD][AD]{port}[/AD]");

    let sleep_duration = Duration::from_millis(INTERVAL_MILLIS);

    println!("Broadcast {msg:?} every {sleep_duration:?}...");

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    loop {
        if let Err(err) = socket.send_to(msg.as_bytes(), &BROADCAST_ADDR) {
            eprintln!("{BROADCAST_ADDR}: {err}");
        }
        sleep(sleep_duration);
    }
}
