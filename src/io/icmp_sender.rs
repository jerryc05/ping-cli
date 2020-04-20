use std::net::{SocketAddr, IpAddr};
use std::time::{Duration, Instant};
use std::io::ErrorKind;
use std::ops::Try;
use std::borrow::Cow;
use std::thread::sleep;
use socket2::{Socket, Domain, Type, Protocol};
use crate::icmp::known_structs::echo::EchoIcmp;
use crate::icmp::icmp_0_trait::Icmp;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use crate::io::dns::dns_resolve;
use crate::utils::{is_debug, MyErr, CONSOLE_FMT_WIDTH};

pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_secs(4);
const DEFAULT_PACKET_SIZE: u16 = 0;
const DEFAULT_TTL: u32 = 64;
const DEFAULT_INTERVAL: Duration = Duration::from_secs(1);
#[allow(dead_code)]
const ETHERNET_V2_HEADER_SIZE: usize = 14;
const IPV4_HEADER_SIZE: usize = 20;


pub fn ping(host_or_ip: &str, timeout_opt: Option<f32>,
            mut count_opt: Option<usize>, interval_opt: Option<f32>,
            p_size_opt: Option<u16>, ttl_opt: Option<u32>) -> Result<(), MyErr> {
  // inspect params
  if is_debug() {
    dbg!(count_opt, interval_opt, p_size_opt, ttl_opt);
  }

  // parse addr
  let addr = match host_or_ip.parse() {
    Ok(ip) => ip,
    Err(_) => dns_resolve(host_or_ip)?
  };

  let p_size = p_size_opt.unwrap_or(DEFAULT_PACKET_SIZE);
  let payload_vec;
  {
    let mut vec = Vec::with_capacity(p_size as usize);
    unsafe { vec.set_len(p_size as usize); }
    payload_vec = vec;
    if is_debug() {
      dbg!(&payload_vec);
    }
  }

  // parse timeout_opt
  let timeout = {
    match timeout_opt {
      Some(t_out) => {
        if t_out >= 0. {
          Some(Duration::from_secs_f32(t_out))
        } else {
          None
        }
      }
      None => Some(DEFAULT_TIMEOUT)
    }
  };

  // print before PING
  {
    println!();
    println!("PING {} ({}) {}({}) bytes of data.",
             host_or_ip, addr, p_size, p_size + 28);
  }

  if count_opt != Some(0) {
    let interval = match interval_opt {
      Some(dur) if dur >= 0. => Duration::from_secs_f32(dur),
      _ => DEFAULT_INTERVAL
    };

    loop {
      // generate icmp struct
      let mut echo_icmp = {
        match addr {
          IpAddr::V4(_) => EchoIcmp::from_payload_v4(&payload_vec),
          IpAddr::V6(_) => EchoIcmp::from_payload_v6(&payload_vec),
        }
      };
      if is_debug() {
        dbg!(&echo_icmp);
      }

      ping_from_ip(addr, timeout, ttl_opt, &mut echo_icmp)?;

      // interval
      {
        if let Some(mut count) = count_opt {
          count -= 1;
          count_opt = Some(count);

          if count == 0 {
            break;
          }
        };

        if is_debug() {
          dbg!(format_args!("Sleeping for [{}] s.", interval.as_secs_f32()));
        }
        sleep(interval);
      }
    }
  }

  println!();
  println!("{1:-^0$}", CONSOLE_FMT_WIDTH, "PING Stopped, Statictics Omitted");

  Ok(())
}


fn ping_from_ip(addr: IpAddr, timeout: Option<Duration>, ttl_opt: Option<u32>,
                echo_icmp: &mut EchoIcmp) -> Result<(), MyErr> {
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

    Socket::new(domain, Type::raw(), protocol).map_err(|err|
      if err.kind() == ErrorKind::PermissionDenied {
        MyErr::from_str(
          "Permission Denied. Perhaps \"setcap cap_net_raw,cap_net_admin=eip\" or \"sudo\" is required.",
          file!(), line!() - 4)
      } else {
        MyErr::from_err(&err, file!(), line!() - 7)
      }
    )?
  };
  let timer;
  let duration;
  let mut recv_buf;

  /* send */
  {
    // timeout
    socket.set_read_timeout(timeout).map_err(
      |err| MyErr::from_err(&err, file!(), line!() - 1))?;

    // ttl
    {
      let ttl = ttl_opt.unwrap_or(DEFAULT_TTL);
      socket.set_ttl(ttl).unwrap_or_else(
        |err| {
          eprintln!("WARN: Failed to set socket TTL.");
          eprintln!("     [{:?}]", err);
          eprintln!("at [{}：{}]", file!(), line!() - 4);
        });
    }

    // checksum
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

    // buffers
    let send_buf;
    {
      send_buf = Vec::from(echo_icmp as &dyn Icmp);

      // configure recv_buf
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

  // recv
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
            print!("{:02x} ", b);
          }
          println!();
        }

        println!("{} bytes from {}, icmp_seq: {:>3}, ttl: {}, rtt: {:.3} ms",
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

    Err(err) =>
      if err.kind() == ErrorKind::TimedOut || err.kind() == ErrorKind::WouldBlock {
        if is_debug() {
          dbg!(err);
        }
        print!("Request timed out ");
        match timeout {
          Some(dur) => println!("after [{}] s.", dur.as_secs_f32()),
          None => println!("indefinitely."),
        }
      } else {
        return Err(MyErr::from_err(&err, file!(), line!() - 4));
      }
  };

  Ok(())
}

#[test]
fn test_ipv4() -> Result<(), MyErr> {
  use std::net::Ipv4Addr;
  ping(&Ipv4Addr::LOCALHOST.to_string(), None,
       Some(1), None, None, None)?;
  Ok(())
}

#[test]
fn test_ipv6() -> Result<(), MyErr> {
  use std::net::Ipv6Addr;
  ping(&Ipv6Addr::LOCALHOST.to_string(), None,
       Some(1), None, None, None)?;
  Ok(())
}