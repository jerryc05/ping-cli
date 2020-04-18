use ping_cli::icmp::known_structs::echo::EchoIcmpV4;
use ping_cli::io::icmp_sender::{ping, PingTimeout};
use ping_cli::utils::MyErr;
use std::env::args;
use std::net::Ipv4Addr;

fn main() -> Result<(), MyErr> {
  /* parse addr */
  let addr = {
    let mut args = args();
    args.next();

    match args.next() {
      Some(addr_str) => addr_str.parse().map_err(
        |err| MyErr::from((&err, file!(), line!() - 1)))?,
      None => {
        eprintln!("No IP address specified! Using default [1.1.1.1]!");
        Ipv4Addr::new(1, 1, 1, 1).into()
      }
    }
  };

  let mut icmp = EchoIcmpV4::from_payload_v4([].as_ref());

  ping(addr,//todo
       PingTimeout::default(),
       None, &mut icmp)?;

  Ok(())
}