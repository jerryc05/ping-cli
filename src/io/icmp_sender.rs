use std::net::{SocketAddr, IpAddr};
use std::time::Duration;
use crate::icmp::icmp_0_trait::Icmp;
use socket2::{Socket, Domain, Type, Protocol};
use std::io::Error;
use crate::utils::is_debug;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use std::ops::Deref;

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
  pub fn new(dur: Duration) -> Self {
    Self(Some(dur))
  }
  pub const fn forever() -> Self {
    Self(None)
  }
}

pub fn ping(addr: IpAddr, timeout_opt: PingTimeout,
            ttl_opt: Option<u32>, icmp: &mut dyn Icmp,
) -> Result<(), (Error, &'static str, u32)> {
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
    Socket::new(domain, Type::raw(), protocol)
      .map_err(|err| (err, file!(), line!()))?
  };

  /* send */
  let len;
  {
    let ttl = ttl_opt.unwrap_or(DEFAULT_TTL);
    socket.set_ttl(ttl).map_err(|err| (err, file!(), line!()))?;

    let timeout = {
      if let Some(dur) = timeout_opt.0 {
        if dur == Duration::from_secs(0) { None } else { timeout_opt.0 }
      } else {
        DEFAULT_TIMEOUT
      }
    };
    socket.set_read_timeout(timeout).map_err(|err| (err, file!(), line!()))?;

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

    let send_buf = Vec::from(icmp as &dyn Icmp);
    len = send_buf.len();

    let dest = SocketAddr::new(addr, 0);
    let size = socket.send_to(&send_buf, &dest.into())
                     .map_err(|err| (err, file!(), line!()))?;
    debug_assert_eq!(size, len);
  }

  /* recv */{
    let mut recv_buf = {
      let capacity = len + IPV4_HEADER_SIZE;
      let mut vec = Vec::with_capacity(capacity);
      unsafe { vec.set_len(capacity); }
      debug_assert!(!vec.as_slice().is_empty());
      vec
    };

    let (size, addr) = socket.recv_from(&mut recv_buf)
                             .map_err(|err| (err, file!(), line!()))?;
    dbg!(size,addr);
    debug_assert_eq!(size, recv_buf.len());

    for b in &recv_buf[0..size] {
      print!("{:02x} ", *b);
    }
    println!();
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

  Ok(())
}
