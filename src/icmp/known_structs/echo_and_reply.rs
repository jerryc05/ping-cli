use std::borrow::Cow;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use crate::icmp::icmp_1_header_1_code::IcmpCode;
use crate::icmp::icmp_0_trait::Icmp;
use crate::icmp::icmp_1_header_0_type::IcmpType;
use std::mem::size_of;

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

pub struct EchoIcmp<'a>(EchoAndReplyIcmp<'a>);

impl Icmp for EchoIcmp<'_> {
  fn type_(&self) -> IcmpType {
    IcmpType::Echo
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

pub struct EchoReplyIcmp<'a>(EchoAndReplyIcmp<'a>);

impl Icmp for EchoReplyIcmp<'_> {
  fn type_(&self) -> IcmpType {
    IcmpType::EchoReply
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