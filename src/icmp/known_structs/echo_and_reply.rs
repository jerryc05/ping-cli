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
struct EchoAndReplyIcmp<'a> {
  checksum: Option<IcmpChecksum>,
  identifier: IdentifierType,
  sequence_num: SequenceNumType,
  payload: Cow<'a, [u8]>,
}

impl EchoAndReplyIcmp<'_> {
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

// Separator

pub struct EchoIcmpV4<'a>(EchoAndReplyIcmp<'a>);

impl Icmp for EchoIcmpV4<'_> {
  fn type_(&self) -> IcmpType {
    V4(IcmpTypeV4::Echo)
  }

  fn code(&self) -> IcmpCode {
    self.0.code()
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.0.checksum()
  }

  fn checksum_mut(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

pub struct EchoReplyIcmpV4<'a>(EchoAndReplyIcmp<'a>);

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

  fn checksum_mut(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

// Separator


pub struct EchoIcmpV6<'a>(EchoAndReplyIcmp<'a>);

impl Icmp for EchoIcmpV6<'_> {
  fn type_(&self) -> IcmpType {
    V6(IcmpTypeV6::EchoRequest)
  }

  fn code(&self) -> IcmpCode {
    self.0.code()
  }

  fn checksum(&self) -> Option<IcmpChecksum> {
    self.0.checksum()
  }

  fn checksum_mut(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}

pub struct EchoReplyIcmpV6<'a>(EchoAndReplyIcmp<'a>);

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

  fn checksum_mut(&mut self, checksum: Option<IcmpChecksum>) {
    self.0.checksum_mut(checksum)
  }

  fn data<'a>(&self) -> Cow<'a, [u8]> {
    self.0.data()
  }
}