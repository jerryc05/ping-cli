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

static SEQUENCE_COUNTER: AtomicU16 = AtomicU16::new(1);

#[derive(Debug)]
struct EchoIcmp<'a> {
  checksum: Option<IcmpChecksum>,
  identifier: u16,
  sequence_num: u16,
  payload: Cow<'a, [u8]>,
}

impl<'a> EchoIcmp<'a> {
  #[inline]
  fn new<T: Into<Cow<'a, [u8]>>>(identifier: u16, payload: T) -> Self {
    EchoIcmp {
      checksum: None,
      identifier,
      sequence_num: SEQUENCE_COUNTER.fetch_add(1, Ordering::Relaxed),
      payload: payload.into(),
    }
  }

  #[inline]
  const fn code(&self) -> IcmpCode {
    IcmpCode(0)
  }

  #[inline]
  const fn checksum(&self) -> Option<IcmpChecksum> {
    self.checksum
  }

  #[inline]
  fn checksum_mut(&mut self, checksum: Option<IcmpChecksum>) {
    self.checksum = checksum;
  }

  fn data(&self) -> Cow<'a, [u8]> {
    let mut vec = Vec::with_capacity(self.payload.len() +
      size_of_val(&self.identifier) + size_of_val(&self.sequence_num)
    );

    /* identifier */ {
      vec.extend(self.identifier.to_be_bytes().iter());
    }

    /* sequence_num */ {
      vec.extend(self.sequence_num.to_be_bytes().iter());
    }

    /* other_data */ {
      vec.extend(self.payload.iter());
    }

    vec.into()
  }
}

// Separator V4

pub struct EchoRequestIcmpV4<'a>(EchoIcmp<'a>);

impl<'a> Icmp<'a> for EchoRequestIcmpV4<'a> {
  fn type_(&self) -> IcmpType {
    V4(IcmpTypeV4::Echo)
  }

  fn code(&self) -> IcmpCode {
    self.0.code()
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.0.checksum()
  }

  fn set_checksum(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

impl<'a> EchoRequestIcmpV4<'a> {
  pub fn new<T: Into<Cow<'a, [u8]>>>(identifier: u16, payload: T) -> Self {
    Self(EchoIcmp::new(identifier, payload))
  }

  pub fn from_payload<T: Into<Cow<'a, [u8]>>>(payload: T) -> Self {
    Self::new(1, payload)
  }
}

pub struct EchoReplyIcmpV4<'a>(EchoIcmp<'a>);

impl<'a> Icmp<'a> for EchoReplyIcmpV4<'a> {
  fn type_(&self) -> IcmpType {
    V4(IcmpTypeV4::EchoReply)
  }

  fn code(&self) -> IcmpCode {
    self.0.code()
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.0.checksum()
  }

  fn set_checksum(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

// Separator V6

pub struct EchoRequestIcmpV6<'a>(EchoIcmp<'a>);

impl<'a> Icmp<'a> for EchoRequestIcmpV6<'a> {
  fn type_(&self) -> IcmpType {
    V6(IcmpTypeV6::EchoRequest)
  }

  fn code(&self) -> IcmpCode {
    self.0.code()
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.0.checksum()
  }

  fn set_checksum(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

impl<'a> EchoRequestIcmpV6<'a> {
  pub fn new<T: Into<Cow<'a, [u8]>>>(identifier: u16, payload: T) -> Self {
    Self(EchoIcmp::new(identifier, payload))
  }

  pub fn from_payload<T: Into<Cow<'a, [u8]>>>(payload: T) -> Self {
    Self::new(1, payload)
  }
}

pub struct EchoReplyIcmpV6<'a>(EchoIcmp<'a>);

impl<'a> Icmp<'a> for EchoReplyIcmpV6<'a> {
  fn type_(&self) -> IcmpType {
    V6(IcmpTypeV6::EchoReply)
  }

  fn code(&self) -> IcmpCode {
    self.0.code()
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.0.checksum()
  }

  fn set_checksum(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}