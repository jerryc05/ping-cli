use ping_cli::{ping, PingTimeout, MyErr};
use std::env::args;
use std::net::Ipv4Addr;

fn main() -> Result<(), MyErr> {
  /* parse addr */
  let mut args = args();
  let host_or_ip = {
    args.next();

    match args.next() {
      Some(str) if str.trim() == "0" => str,
      _ => {
        eprintln!("Using default Ipv4 loop-back!");
        Ipv4Addr::LOCALHOST.to_string()
      }
    }
  };

  /* parse other args */
  let mut count_opt = None;
  let mut p_size_opt = None;
  let mut ttl_opt = None;
  {
    while let Some(arg1) = args.next() {
      match args.next() {
        Some(arg2) =>
          match arg1.as_str() {
            "-c" => count_opt = Some(arg2.parse().map_err(|_|
              MyErr::from_str("Failed to parse [{}] to u16 count!",
                              file!(), line!() - 2))?),
            "-s" => p_size_opt = Some(arg2.parse().map_err(|_|
              MyErr::from_str("Failed to parse [{}] to u16 packet size!",
                              file!(), line!() - 2))?),
            "-t" => ttl_opt = Some(arg2.parse().map_err(|_|
              MyErr::from_str("Failed to parse [{}] to u16 packet size!",
                              file!(), line!() - 2))?),
            _ => {}
          },
        None => return Err(MyErr::from_str(
          format!("This arg did not come in pair: [{}]!", arg1),
          file!(), line!() - 2))
      }
    }
  };

  ping(&host_or_ip, PingTimeout::default(),
       count_opt, p_size_opt, ttl_opt)?;

  Ok(())
}