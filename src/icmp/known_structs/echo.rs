use crate::icmp::icmp_0_trait::Icmp;
use crate::icmp::icmp_1_header_0_type::IcmpType;
use crate::icmp::icmp_1_header_0_type_v4::IcmpTypeV4;
use crate::icmp::icmp_1_header_0_type_v6::IcmpTypeV6;
use crate::icmp::icmp_1_header_1_code::IcmpCode;
use crate::icmp::icmp_1_header_0_type::IcmpType::{V4, V6};
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use std::borrow::Cow;
use std::mem::size_of_val;
use std::sync::atomic::{AtomicU16, Ordering};
use crate::MyErr;
use std::ops::Try;

static SEQUENCE_COUNTER: AtomicU16 = AtomicU16::new(1);

/**
[RFC 792](https://tools.ietf.org/html/rfc792)

  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|         type_         |         code          |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                   checksum                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|      identifier       |        seq_num        |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|   payload ...
+--+--+--+--+--
*/
#[derive(Debug)]
pub struct EchoIcmp<'a> {
  type_: IcmpType,
  checksum: Option<IcmpChecksum>,
  identifier: u16,
  seq_num: u16,
  pub(crate) payload: Cow<'a, [u8]>,
}

impl<'a> Icmp<'a> for EchoIcmp<'a> {
  fn type_(&self) -> IcmpType {
    self.type_
  }

  fn code(&self) -> IcmpCode {
    IcmpCode(0)
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.checksum
  }

  fn set_checksum(&mut self, checksum: Option<IcmpChecksum>) {
    self.checksum = checksum;
  }

  fn data(&self) -> Cow<'a, [u8]> {
    let mut vec = Vec::with_capacity(self.payload.len() +
      size_of_val(&self.identifier) + size_of_val(&self.seq_num)
    );

    /* identifier */ {
      vec.extend(self.identifier.to_be_bytes().iter());
    }

    /* sequence_num */ {
      vec.extend(self.seq_num.to_be_bytes().iter());
    }

    /* other_data */ {
      vec.extend(self.payload.iter());
    }

    vec.into()
  }
}

// V4
impl<'a> EchoIcmp<'a> {
  const REQUEST_TYPE_V4: IcmpType = V4(IcmpTypeV4::Echo);

  pub fn new_v4<T: Into<Cow<'a, [u8]>>>(identifier: u16, payload: T) -> Self {
    Self::new_(Self::REQUEST_TYPE_V4, identifier, payload)
  }

  pub fn from_payload_v4<T: Into<Cow<'a, [u8]>>>(payload: T) -> Self {
    Self::new_(Self::REQUEST_TYPE_V4, 1, payload)
  }

  pub fn from_reply_v4<T: Into<Cow<'a, [u8]>>>(_reply: T) -> Self {
    todo!()
  }
}

// V6
impl<'a> EchoIcmp<'a> {
  const REQUEST_TYPE_V6: IcmpType = V6(IcmpTypeV6::EchoRequest);

  pub fn new_v6<T: Into<Cow<'a, [u8]>>>(identifier: u16, payload: T) -> Self {
    Self::new_(Self::REQUEST_TYPE_V6, identifier, payload)
  }

  pub fn from_payload_v6<T: Into<Cow<'a, [u8]>>>(payload: T) -> Self {
    Self::new_(Self::REQUEST_TYPE_V6, 1, payload)
  }

  pub fn from_reply_v6<T: Into<Cow<'a, [u8]>>>(_reply: T) -> Self {
    todo!()
  }
}

// EchoIcmp impls
impl<'a> EchoIcmp<'a> {
  #[inline]
  fn new_<T: Into<Cow<'a, [u8]>>>(type_: IcmpType, identifier: u16, payload: T) -> Self {
    Self {
      type_,
      checksum: None,
      identifier,
      seq_num: SEQUENCE_COUNTER.fetch_add(1, Ordering::Relaxed),
      payload: payload.into(),
    }
  }

  #[inline]
  pub fn parse_identifier(bytes: &[u8]) -> Result<u16, MyErr> {
    let b4 = bytes.get(4).copied().into_result().map_err(
      |_| MyErr::from_str(
        format!("Failed to get the 4th index from arr [{:?}]!", bytes),
        file!(), line!() - 3))?;

    let b5 = bytes.get(5).copied().into_result().map_err(
      |_| MyErr::from_str(
        format!("Failed to get the 5th index from arr [{:?}]!", bytes),
        file!(), line!() - 3))?;

    Ok(u16::from_be_bytes([b4, b5]))
  }

  #[inline]
  pub fn parse_seq_num(bytes: &[u8])-> Result<u16, MyErr> {
    let b6 = bytes.get(6).copied().into_result().map_err(
      |_| MyErr::from_str(
        format!("Failed to get the 6th index from arr [{:?}]!", bytes),
        file!(), line!() - 3))?;

    let b7 = bytes.get(7).copied().into_result().map_err(
      |_| MyErr::from_str(
        format!("Failed to get the 7th index from arr [{:?}]!", bytes),
        file!(), line!() - 3))?;

    Ok(u16::from_be_bytes([b6, b7]))
  }

  #[inline]
  pub fn parse_payload(bytes: &[u8]) -> &[u8] {
    &bytes[6..]
  }
}
