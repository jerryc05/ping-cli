use ping_cli::icmp::known_structs::echo::EchoRequestIcmpV4;
use ping_cli::io::icmp_sender::{ping, PingTimeout};
use std::io::Error;
use std::net::Ipv4Addr;
use std::time::Duration;

fn main() -> Result<(), (Error, &'static str, u32)> {
  let mut icmp = EchoRequestIcmpV4::from_payload([].as_ref());

  ping(Ipv4Addr::new(1, 1, 1, 1).into(),
       PingTimeout::new(Duration::from_millis(100)),
       None, &mut icmp)?;

  Ok(())
}