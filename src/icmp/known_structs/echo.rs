use std::borrow::Cow;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use crate::icmp::icmp_1_header_1_code::IcmpCode;
use crate::icmp::icmp_0_trait::Icmp;
use crate::icmp::icmp_1_header_0_type_v4::IcmpTypeV4;
use std::mem::size_of;
use crate::icmp::icmp_1_header_0_type::IcmpType;
use crate::icmp::icmp_1_header_0_type::IcmpType::{V4, V6};
use crate::icmp::icmp_1_header_0_type_v6::IcmpTypeV6;

type IdentifierType = u16;
type SequenceNumType = u16;

#[derive(Debug)]
struct EchoIcmp<'a> {
  checksum: Option<IcmpChecksum>,
  identifier: IdentifierType,
  sequence_num: SequenceNumType,
  payload: Cow<'a, [u8]>,
}

impl EchoIcmp<'_> {
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

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    let mut vec = Vec::with_capacity(
      size_of::<IdentifierType>() + size_of::<SequenceNumType>() +
        self.payload.len());

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

impl Icmp for EchoRequestIcmpV4<'_> {
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

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

impl<'a> EchoRequestIcmpV4<'a> {
  pub fn new<T: Into<Cow<'a, [u8]>>>(identifier: IdentifierType,
                                     sequence_num: SequenceNumType,
                                     payload: T) -> Self {
    Self(EchoIcmp {
      checksum: None,
      identifier,
      sequence_num,
      payload: payload.into(),
    })
  }
  pub fn from_payload<T: Into<Cow<'a, [u8]>>>(payload: T) -> Self {
    Self::new(0, 0, payload)
  }
}

pub struct EchoReplyIcmpV4<'a>(EchoIcmp<'a>);

impl Icmp for EchoReplyIcmpV4<'_> {
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

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

// Separator V6

pub struct EchoRequestIcmpV6<'a>(EchoIcmp<'a>);

impl Icmp for EchoRequestIcmpV6<'_> {
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

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

impl<'a> EchoRequestIcmpV6<'a> {
  pub const fn new(identifier: IdentifierType,
                   sequence_num: SequenceNumType, payload: Cow<'a, [u8]>, ) -> Self {
    Self(EchoIcmp { checksum: None, identifier, sequence_num, payload })
  }
}

pub struct EchoReplyIcmpV6<'a>(EchoIcmp<'a>);

impl Icmp for EchoReplyIcmpV6<'_> {
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

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}