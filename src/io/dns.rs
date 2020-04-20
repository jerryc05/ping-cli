use std::net::IpAddr;
use std::process::Command;
use std::ops::Try;
use crate::MyErr;

pub(crate) fn dns_resolve(host: &str) -> Result<IpAddr, MyErr> {
  let response = {
    let output = Command::new("host")
      .arg("-t").arg("a").arg(host).output().map_err(
      |err| MyErr::from_str(
        format!("Failed to execute \"host\" cmd! Err: [{:?}]", err),
        file!(), line!() - 4))?;

    if !output.status.success() {
      return Err(MyErr::from_str(format!(
        "Failed to resolve DNS for host [{}]! Note that Rust's stdlib does not support \
parsing abbreviated ip addresses (like a.b or a.b.c) yet. It uses \"inet_pton()\" internally.",
        host), file!(), line!() - 4));
    }

    String::from_utf8_lossy(&output.stdout).into_owned()
  };
  dbg!(&response);

  let addr_str = {
    // Start point
    let addr_idx = {
      let keyword = "has address";
      keyword.len() + response.find(keyword).into_result().map_err(
        |_| MyErr::from_str(
          format!("Cannot locate \"{}\" in dns result [{}]!", keyword, response),
          file!(), line!() - 3))?
    };

    let mut data = &response.as_str()[addr_idx..];

    // Advance if whitespaces are found
    while unsafe { data.as_bytes().get_unchecked(0) } == &b' ' {
      data = &data[1..]
    }

    // End point
    let end_idx = {
      data.find('\n').into_result().map_err(
        |_| MyErr::from_str(
          format!("Cannot locate end of \"data\" in dns result [{}]!", response),
          file!(), line!() - 3))?
    };

    dbg!(&data[..end_idx])
  };

  dbg!(Ok(addr_str.parse().map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?))
}

#[cfg(feature = "dns")]
#[test]
fn test_dns_resolve() {
  assert!(dns_resolve("www.google.com").is_ok());
}