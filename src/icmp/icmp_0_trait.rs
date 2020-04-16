use crate::icmp::icmp_1_header_0_type::IcmpType;
use crate::icmp::icmp_1_header_1_code::IcmpCode;
use crate::icmp::icmp_1_header_2_checksum::IcmpChecksum;
use std::borrow::Cow;

/**
[RFC 792](https://tools.ietf.org/html/rfc792)

  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|         type_         |         code          |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                   checksum                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|    data ...
+--+--+--+--+--
*/
pub trait Icmp<'a> {
  fn type_(&self) -> IcmpType;

  fn code(&self) -> IcmpCode;

  fn checksum(&self) -> Option<IcmpChecksum>;
  fn set_checksum(&mut self, checksum: Option<IcmpChecksum>);

  fn data(&self) -> Cow<'a, [u8]>;
}

impl<'a> From<&dyn Icmp<'a>> for Vec<u8> {
  fn from(icmp: &dyn Icmp) -> Self {
    let mut result = Self::with_capacity(4);

    /* type */ {
      result.push((&icmp.type_()).into())
    }

    /* code */ {
      result.push(*icmp.code());
    }

    /* checksum */ {
      let bytes: [u8; 2] = {
        if let Some(checksum) = &icmp.checksum() {
          checksum.to_be_bytes()
        } else {
          0_u16.to_be_bytes()
        }
      };
      result.extend(bytes.iter())
    }

    /* data */ {
      result.extend(icmp.data().iter());
    }

    result
  }
}

#[test]
fn test_icmp_to_vec() {
  // todo
}