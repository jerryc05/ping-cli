use std::net::IpAddr;
use crate::MyErr;
use std::process::Command;
use std::ops::Try;

pub(crate) fn dns_resolve(host: &str) -> Result<IpAddr, MyErr> {
  let response = {
    let output = Command::new("host")
      .arg("-t").arg("a").arg(host).output().map_err(
      |err| MyErr::from_str(
        format!("Failed to execute \"host\" cmd! Err: [{:?}]", err),
        file!(), line!() - 4))?;

    if !output.status.success() {
      return Err(MyErr::from_str(
        format!("Failed to resolve DNS for host [{:?}]!", host),
        file!(), line!() - 1));
    }

    String::from_utf8_lossy(&output.stdout).into_owned()
  };

  let addr_str = {
    // Start point
    let addr_idx = {
      let keyword = "\"has address\":";
      keyword.len() + response.find(keyword).into_result().map_err(
        |_| MyErr::from_str(
          format!("Cannot locate \"has address\" in dns result [{}]!", response),
          file!(), line!() - 3))?
    };

    let mut data = &response.as_str()[addr_idx..];

    // Advance if whitespaces are found
    while unsafe { data.as_bytes().get_unchecked(0) } == &b' ' {
      data = &data[1..]
    }

    // End point
    let space_idx = {
      data.find(' ').into_result().map_err(
        |_| MyErr::from_str(
          format!("Cannot locate end of \"data\" in dns result [{}]!", response),
          file!(), line!() - 3))?
    };

    dbg!(&data[..space_idx])
  };

  dbg!(Ok(addr_str.parse().map_err(
    |err| MyErr::from_err(&err, file!(), line!() - 1))?))
}

#[cfg(feature = "dns")]
#[test]
fn test_dns_resolve() {
  assert!(dns_resolve("www.google.com").is_ok());
}