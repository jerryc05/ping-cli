use std::net::IpAddr;
use crate::MyErr;
use crate::io::icmp_sender::DEFAULT_TIMEOUT;
use std::ops::Try;
use crate::utils::is_debug;

pub(crate) fn dns_resolve(host: &str) -> Result<IpAddr, MyErr> {
  let response = {
    let raw = minreq::get(
      &format!("https://cloudflare-dns.com/dns-query?name={}&type=A", host.trim())
    ).with_header("accept", "application/dns-json")
     .with_timeout(DEFAULT_TIMEOUT.as_secs()).send().map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;

    String::from_utf8_lossy(raw.as_bytes()).into_owned()
  };

  let addr_str = {
    let keyword = "\"data\":";
    let data_idx = keyword.len() + response.find(keyword).into_result().map_err(
      |_| MyErr::from_str(
        format!("Cannot locate \"data\" in dns result [{}]!", response),
        file!(), line!() - 3))?;

    let mut data = &response.as_str()[data_idx..];

    let start_idx = 1 + data.find('"').into_result().map_err(
      |_| MyErr::from_str(
        format!("Cannot locate start of \"data\" in dns result [{}]!", response),
        file!(), line!() - 3))?;
    data = &data[start_idx..];

    let end_idx = data.find('"').into_result().map_err(
      |_| MyErr::from_str(
        format!("Cannot locate end of \"data\" in dns result [{}]!", response),
        file!(), line!() - 3))?;
    &data[..end_idx]
  };

  if is_debug() {
    println!("{} -> {}", host, addr_str);
  }

  Ok(addr_str.parse().map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?)
}

#[test]
fn test_dns_resolve() {
  assert!(dns_resolve("www.google.com").is_ok());
}