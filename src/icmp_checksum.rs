use std::ops::{Deref, DerefMut};
use crate::icmp::Icmp;
use std::mem::size_of;

pub struct IcmpChecksum(u16);

impl Deref for IcmpChecksum {
  type Target = u16;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for IcmpChecksum {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub struct ChecksumIsNotNoneError;

impl IcmpChecksum {
  pub(crate) unsafe fn checksum_unchecked(icmp: &mut Icmp) {
    #[inline]
    fn checksum_add(num: &mut u16, adder: u16) {
      let (res, of) = (*num).overflowing_add(adder);
      *num = res;
      if of {
        checksum_add(num, 1);
      }
    }

    let mut result = 0;
    let vec_icmp = Vec::from(icmp as &Icmp);
    let mut iter = vec_icmp.chunks_exact(2);
    let shift = 8 * size_of::<u8>() as u16;

    /* chunks */ {
      while let Some(arr) = iter.next() {
        debug_assert!(arr.len() == 2);
        checksum_add(&mut result, u16::from_be_bytes(
          [*arr.get_unchecked(0), *arr.get_unchecked(1)]
        ))
      }
    }

    /* remainder */ {
      let rem: &[u8] = iter.remainder();
      if !rem.is_empty() {
        debug_assert!(rem.len() == 1);
        let byte = *rem.get_unchecked(0) as u16;
        checksum_add(&mut result, byte << shift);
      }
    }

    icmp.checksum = Some(Self(!result));
  }
}