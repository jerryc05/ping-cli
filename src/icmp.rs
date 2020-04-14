use crate::icmp_type::IcmpType;
use crate::icmp_code::IcmpCode;
use crate::icmp_checksum::{IcmpChecksum, ChecksumIsNotNoneError};

/**
[RFC 792](https://tools.ietf.org/html/rfc792)

  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|         Type          |         Code          |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
|                   Checksum                    |
+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
*/
pub struct Icmp {
  pub type_: IcmpType,
  pub code: IcmpCode,
  pub checksum: Option<IcmpChecksum>,
}

impl Icmp {
  pub fn checksum(&mut self) -> Result<(), ChecksumIsNotNoneError> {
    if self.checksum.is_some() {
      return Err(ChecksumIsNotNoneError);
    }

    unsafe { self.checksum_unchecked(); }
    Ok(())
  }

  pub unsafe fn checksum_unchecked(&mut self) {
    IcmpChecksum::checksum_unchecked(self);
  }
}

impl Into<Vec<u8>> for Icmp {
  fn into(self) -> Vec<u8> {
    let mut result = Vec::with_capacity(4);

    /* type */ {
      result.push(self.type_.into())
    }

    /* code */ {
      result.push(self.code.into())
    }

    /* checksum */ {
      let bytes: [u8; 2] = {
        if let Some(checksum) = self.checksum {
          checksum.to_be_bytes()
        } else {
          0_u16.to_be_bytes()
        }
      };
      result.extend_from_slice(&bytes)
    }

    unimplemented!();
    result
  }
}