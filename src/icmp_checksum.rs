use std::ops::{Deref, DerefMut};
use crate::icmp::Icmp;

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
    let vec_icmp = Vec::from(icmp as &Icmp);
    icmp.checksum = Some(Self(checksum_impl(vec_icmp.as_slice())));
  }
}

fn checksum_impl(slice: &[u8]) -> u16 {
  #[inline]
  fn checksum_add(num: &mut u16, adder: u16) {
    let (res, of) = (*num).overflowing_add(adder);
    *num = res;
    if of {
      checksum_add(num, 1);
    }
  }

  let mut result = 0;
  let mut iter = slice.chunks_exact(2);

  /* chunks */ {
    while let Some(arr) = iter.next() {
      debug_assert!(arr.len() == 2);
      checksum_add(&mut result, u16::from_be_bytes(
        unsafe { [*arr.get_unchecked(0), *arr.get_unchecked(1)] }
      ))
    }
  }

  /* remainder */ {
    let rem: &[u8] = iter.remainder();
    if !rem.is_empty() {
      debug_assert!(rem.len() == 1);
      checksum_add(&mut result, u16::from_be_bytes(
        unsafe { [*rem.get_unchecked(0), 0] }
      ))
    }
  }

  !result
}

#[test]
fn test_checksum_impl(){

}