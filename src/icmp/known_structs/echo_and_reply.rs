use std::borrow::Cow;
use crate::icmp::Icmp;
use crate::icmp_1_code::IcmpCode;
use crate::icmp_2_checksum::IcmpChecksum;
use crate::icmp_0_type::IcmpType;

#[derive(Debug)]
struct EchoAndReplyIcmp<'a> {
  checksum: Option<IcmpChecksum>,
  identifier: u8,
  sequence_num: u8,
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
    let mut vec = Vec::with_capacity(2 + self.payload.len());

    /* identifier */ {
      vec.push(self.identifier);
    }

    /* sequence_num */ {
      vec.push(self.sequence_num);
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