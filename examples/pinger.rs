use ping_cli::{ping, PingTimeout, MyErr, EchoIcmp};
use std::env::args;
use std::net::{Ipv4Addr, IpAddr};

fn main() -> Result<(), MyErr> {
  /* parse addr */
  let addr = {
    let mut args = args();
    args.next();

    match args.next() {
      Some(addr_str) => addr_str.parse().map_err(
        |err| MyErr::from_err(&err, file!(), line!() - 1))?,
      None => {
        eprintln!("No IP address specified! Using default [1.1.1.1]!");
        Ipv4Addr::LOCALHOST.into()
      }
    }
  };

  /* generate icmp struct */
  let mut echo_icmp = {
    match addr {
      IpAddr::V4(_) => EchoIcmp::from_payload_v4([].as_ref()),
      IpAddr::V6(_) => EchoIcmp::from_payload_v6([].as_ref()),
    }
  };

  ping(addr, PingTimeout::default(), None, &mut echo_icmp)?;

  Ok(())
}