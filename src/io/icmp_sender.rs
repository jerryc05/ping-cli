use crate::icmp::icmp_0_trait::Icmp;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use crate::utils::{is_debug, MyErr};
use socket2::{Socket, Domain, Type, Protocol};
use std::net::{SocketAddr, IpAddr};
use std::ops::Deref;
use std::time::{Duration, Instant};
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

impl Deref for PingTimeout {
  type Target = Option<Duration>;

  fn deref(&self) -> &Self::Target {
    &self.0
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

pub fn ping(addr: IpAddr, timeout_opt: PingTimeout,
            ttl_opt: Option<u32>, icmp: &mut dyn Icmp,
) -> Result<Duration, MyErr> {
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
      |err| MyErr::from((&err, file!(), line!())))?
  };
  let timer;
  let duration;

  /* send */
  let mut recv_buf;
  {
    let ttl = ttl_opt.unwrap_or(DEFAULT_TTL);
    socket.set_ttl(ttl).map_err(
      |err| MyErr::from((&err, file!(), line!() - 1)))?;

    let timeout = {
      if let Some(dur) = timeout_opt.0 {
        if dur == Duration::from_secs(0) { None } else { timeout_opt.0 }
      } else {
        DEFAULT_TIMEOUT
      }
    };
    socket.set_read_timeout(timeout).map_err(
      |err| MyErr::from((&err, file!(), line!() - 1)))?;

    /* checksum */
    {
      if is_debug() {
        IcmpChecksum::gen_checksum(icmp).unwrap_or_else(|_| {
          eprintln!("Unexpected overwriting checksum! Please report this bug!");
          IcmpChecksum::override_checksum(icmp);
        });
      } else {
        IcmpChecksum::override_checksum(icmp);
      }
    }

    /* buffers */
    let send_buf;
    {
      send_buf = Vec::from(icmp as &dyn Icmp);

      {
        let capacity = IPV4_HEADER_SIZE + send_buf.len();
        recv_buf = Vec::with_capacity(capacity);
        unsafe { recv_buf.set_len(capacity); }
        debug_assert!(!recv_buf.as_slice().is_empty());
      };
    }

    let dest = SocketAddr::new(addr, 0);
    timer = Instant::now();
    let size = socket.send_to(&send_buf, &dest.into()).map_err(
      |err| MyErr::from((&err, file!(), line!())))?;
    debug_assert_eq!(size, send_buf.len());
  }

  /* recv */{
    let (size, addr) = socket.recv_from(&mut recv_buf).map_err(
      |err| MyErr::from((&err, file!(), line!() - 1)))?;
    duration = timer.elapsed();
    debug_assert_eq!(size, recv_buf.len());

    for b in &recv_buf[IPV4_HEADER_SIZE..size] {
      print!("{:02x} ", *b);
    }
    println!();
    println!("64 bytes from 93.184.216.34: icmp_seq=0 ttl=56 time=11.632 ms");
    println!("{} bytes from {}: icmp_seq={} ttl={} time={:.3} ms",
             "??",
             addr.as_std().map_or_else(
               || Cow::from("??"),
               |std_addr| std_addr.ip().to_string().into()),
             "??",
             socket.ttl().map_or_else(
               |err| {
                 eprintln!("{:?}", MyErr::from((&err, file!(), line!() - 2)));
                 Cow::from("??")
               },
               |ttl| ttl.to_string().into()),
             duration.as_secs_f32() * (Duration::from_secs(1).as_millis() as f32));
  }

  // let reply =
  //   if dest.is_ipv4() {
  //     let ipv4_packet =
  //       match IpV4Packet::decode(&buffer) {
  //         Ok(packet) => packet,
  //         Err(_) => return Err(ErrorKind::InternalError.into()),
  //       };
  //     match EchoReply::decode::<IcmpV4>(ipv4_packet.data) {
  //       Ok(reply) => reply,
  //       Err(_) => return Err(ErrorKind::InternalError.into()),
  //     }
  //   } else {
  //     match EchoReply::decode::<IcmpV6>(&buffer) {
  //       Ok(reply) => reply,
  //       Err(_) => return Err(ErrorKind::InternalError.into()),
  //     }
  //   };

  Ok(duration)
}
