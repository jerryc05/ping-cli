use ping_cli::{ping, PingTimeout, MyErr};
use std::env::args;
use std::net::Ipv4Addr;

fn main() -> Result<(), MyErr> {
  /* parse addr */
  let mut args = args();
  let host_or_ip = {
    args.next();

    match args.next() {
      Some(str) => str,
      None => {
        eprintln!("No IP address specified! Using default Ipv4 loop-back!");
        Ipv4Addr::LOCALHOST.to_string()
      }
    }
  };

  /* parse size */
  let size = {
    match args.next() {
      Some(u16_str) => u16_str.parse().map_err(|_|
        MyErr::from_str("Failed to parse [{}] to u16. Check your input!",
                        file!(), line!() - 2))?,
      None => 0
    }
  };

  ping(&host_or_ip, size, PingTimeout::default(), None)?;

  Ok(())
}