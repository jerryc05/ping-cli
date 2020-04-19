use crate::icmp::known_structs::echo::EchoIcmp;
use crate::icmp::icmp_0_trait::Icmp;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use crate::utils::{is_debug, MyErr, CONSOLE_FMT_WIDTH};
use socket2::{Socket, Domain, Type, Protocol};
use std::net::{SocketAddr, IpAddr};
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::ops::Try;
use std::borrow::Cow;

const DEFAULT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(4));
const DEFAULT_TTL: u32 = 64;
#[allow(dead_code)]
const ETHERNET_V2_HEADER_SIZE: usize = 14;
const IPV4_HEADER_SIZE: usize = 20;

#[derive(Debug)]
pub struct PingTimeout(Option<Duration>);

impl Default for PingTimeout {
  fn default() -> Self {
    Self(DEFAULT_TIMEOUT)
  }
}

impl PingTimeout {
  pub fn from_ms(ms: u64) -> Self {
    Self::from_duration(Duration::from_millis(ms))
  }

  pub fn from_duration(dur: Duration) -> Self {
    Self(Some(dur))
  }

  pub const fn forever() -> Self {
    Self(None)
  }
}

// Separator

pub fn ping(addr: IpAddr, timeout_opt: PingTimeout,
            ttl_opt: Option<u32>, echo_icmp: &mut EchoIcmp) -> Result<(), MyErr> {
  let socket = {
    let domain;
    let protocol;
    match addr {
      IpAddr::V4(_) => {
        domain = Domain::ipv4();
        protocol = Some(Protocol::icmpv4());
      }
      IpAddr::V6(_) => {
        domain = Domain::ipv6();
        protocol = Some(Protocol::icmpv6());
      }
    };
    Socket::new(domain, Type::raw(), protocol).map_err(
      |err| if err.kind() == ErrorKind::PermissionDenied {
        MyErr::from_str(
          "Permission Denied. \
Perhaps \"setcap cap_net_raw,cap_net_admin=eip\" or \"sudo\" is required.",
          file!(), line!() - 5)
      } else {
        MyErr::from_err(&err, file!(), line!() - 7)
      }
    )?
  };
  let timer;
  let duration;

  /* send */
  let mut recv_buf;
  {
    let timeout = {
      if let Some(dur) = timeout_opt.0 {
        if dur == Duration::from_secs(0) { None } else { timeout_opt.0 }
      } else {
        DEFAULT_TIMEOUT
      }
    };
    socket.set_read_timeout(timeout).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;

    let ttl = ttl_opt.unwrap_or(DEFAULT_TTL);
    socket.set_ttl(ttl).unwrap_or_else(
      |err| {
        eprintln!("WARN: Failed to set socket TTL.");
        eprintln!("     [{:?}]", err);
        eprintln!("at [{}：{}]", file!(), line!() - 4);
      });

    /* checksum */
    {
      if is_debug() {
        IcmpChecksum::gen_checksum(echo_icmp).unwrap_or_else(|_| {
          eprintln!("Unexpected overwriting checksum! Please report this bug!");
          IcmpChecksum::override_checksum(echo_icmp);
        });
      } else {
        IcmpChecksum::override_checksum(echo_icmp);
      }
    }

    /* print before PING */
    {
      println!();
      let bytes = echo_icmp.payload.len();
      println!("PING {} ({}) {}({}) bytes of data.",
               addr.to_string(), // todo
               addr.to_string(),
               bytes, bytes + 28);
    }

    /* buffers */
    let send_buf;
    {
      send_buf = Vec::from(echo_icmp as &dyn Icmp);

      /* configure recv_buf */
      {
        let capacity = match addr {
          IpAddr::V4(_) => send_buf.len() + IPV4_HEADER_SIZE,
          IpAddr::V6(_) => send_buf.len(),
        };
        recv_buf = Vec::with_capacity(capacity);
        unsafe { recv_buf.set_len(capacity); }
        debug_assert!(!recv_buf.as_slice().is_empty());
      };
    }

    let dest = SocketAddr::new(addr, 0);
    timer = Instant::now();
    let size = socket.send_to(&send_buf, &dest.into()).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;
    debug_assert_eq!(size, send_buf.len());
  }

  /* recv */
  match socket.recv_from(&mut recv_buf) {
    Ok((size, sock_addr)) => {
      duration = timer.elapsed();
      debug_assert_eq!(size, recv_buf.len());

      let icmp_recv = match addr {
        IpAddr::V4(_) => &recv_buf[IPV4_HEADER_SIZE..size],
        IpAddr::V6(_) => &recv_buf,
      };

      /* output */
      {
        if is_debug() {
          for b in icmp_recv {
            print!("{:02x} ", *b);
          }
          print!("\t");
        }
        println!("{} bytes from {}: icmp_seq={} ttl={} time={:.3} ms",
                 icmp_recv.len(),
                 sock_addr.as_std().into_result().map_err(
                   |err| MyErr::from_err(&err, file!(), line!() - 1))?
                   .ip(),
                 EchoIcmp::parse_seq_num(icmp_recv).into_result().map_err(
                   |err| MyErr::from_err(&err, file!(), line!() - 1))?,
                 socket.ttl().map_or_else(|_| Cow::from("--"),
                                          |ttl| ttl.to_string().into()),
                 duration.as_secs_f32() * (Duration::from_secs(1).as_millis() as f32));
      }
    }

    Err(err) => if err.kind() == ErrorKind::TimedOut {
      println!("Request timed out.");
    } else {
      return Err(MyErr::from_err(&err, file!(), line!() - 1));
    }
  };

  /*

  --- 1.1.1.1 ping statistics ---
  3 packets transmitted, 3 received, 0% packet loss, time 2002ms
  rtt min/avg/max/mdev = 1.413/1.686/2.086/0.289 ms

  */

  println!();
  println!("{1:-^0$}", CONSOLE_FMT_WIDTH, "PING stopped here.");

  Ok(())
}

#[test]
fn test_ipv4() -> Result<(), MyErr> {
  use std::net::Ipv4Addr;
  let mut echo_icmp = EchoIcmp::from_payload_v4([].as_ref());
  ping(Ipv4Addr::LOCALHOST.into(),
       PingTimeout::default(),
       None, &mut echo_icmp)?;
  Ok(())
}

#[test]
fn test_ipv6() -> Result<(), MyErr> {
  use std::net::Ipv6Addr;
  let mut echo_icmp = EchoIcmp::from_payload_v6([].as_ref());
  ping(Ipv6Addr::LOCALHOST.into(),
       PingTimeout::default(),
       None, &mut echo_icmp)?;
  Ok(())
}