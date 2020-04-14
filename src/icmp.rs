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
pub trait Icmp<'a> {
  fn type_(&self) -> IcmpType;
  // fn type_mut(&mut self, type_: IcmpType);

  fn code(&self) -> IcmpCode;
  // fn code_mut(&mut self, code: IcmpCode);

  fn checksum(&self) -> Option<IcmpChecksum>;
  fn checksum_mut(&mut self, checksum: Option<IcmpChecksum>);

  fn data(&self) -> Cow<'a, [u8]>;
  fn data_mut(&mut self, data: Cow<'a, [u8]>);
}

impl<'a> dyn Icmp<'_> {
  pub fn gen_checksum(&mut self) -> Result<(), ChecksumIsNotNoneError> {
    if self.checksum().is_some() {
      return Err(ChecksumIsNotNoneError);
    }

    unsafe { self.gen_checksum_unchecked(); }
    Ok(())
  }

  /// # Safety
  /// This function will not check whether `self.checksum` is None.
  pub unsafe fn gen_checksum_unchecked(&mut self) {
    IcmpChecksum::checksum_unchecked(self);
  }
}

impl<'a> From<&dyn Icmp<'_>> for Vec<u8> {
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