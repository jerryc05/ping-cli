use crate::icmp_0_type::IcmpType;
use crate::icmp_1_code::IcmpCode;
use crate::icmp_2_checksum::{IcmpChecksum, ChecksumIsNotNoneError};
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
pub struct Icmp<'a> {
  pub type_: IcmpType,
  pub code: IcmpCode,
  pub checksum: Option<IcmpChecksum>,
  pub data: Cow<'a, [u8]>,
}

impl<'a> Icmp<'_> {
  pub fn checksum(&mut self) -> Result<(), ChecksumIsNotNoneError> {
    if self.checksum.is_some() {
      return Err(ChecksumIsNotNoneError);
    }

    unsafe { self.checksum_unchecked(); }
    Ok(())
  }

  /// # Safety
  /// This function will not check whether `self.checksum` is None.
  pub unsafe fn checksum_unchecked(&mut self) {
    IcmpChecksum::checksum_unchecked(self);
  }
}

impl<'a> From<&Icmp<'_>> for Vec<u8> {
  fn from(icmp: &Icmp) -> Self {
    let mut result = Self::with_capacity(4);

    /* type */ {
      result.push((&icmp.type_).into())
    }

    /* code */ {
      result.push(*icmp.code);
    }

    /* checksum */ {
      let bytes: [u8; 2] = {
        if let Some(checksum) = &icmp.checksum {
          checksum.to_be_bytes()
        } else {
          0_u16.to_be_bytes()
        }
      };
      result.extend_from_slice(&bytes)
    }

    /* data */ {
      result.extend_from_slice(icmp.data.as_ref());
    }

    result
  }
}

#[test]
fn test_icmp_to_vec(){
  // todo
}