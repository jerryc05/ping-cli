use std::ops::{Deref, DerefMut};
use crate::icmp::icmp_0_trait::Icmp;

#[derive(Debug, Clone, Copy)]
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
  pub fn gen_checksum(icmp: &mut dyn Icmp) -> Result<(), ChecksumIsNotNoneError> {
    if icmp.checksum().is_some() {
      return Err(ChecksumIsNotNoneError);
    }

    unsafe { Self::gen_checksum_unchecked(icmp); }
    Ok(())
  }

  pub fn override_checksum(icmp: &mut dyn Icmp) {
    if icmp.checksum().is_some() {
      icmp.set_checksum(None);
    }

    unsafe { Self::gen_checksum_unchecked(icmp); }
  }

  /// # Safety
  /// This function will not check whether `self.checksum` is None.
  pub unsafe fn gen_checksum_unchecked(icmp: &mut dyn Icmp) {
    debug_assert!(icmp.checksum().is_none());
    let vec_icmp = Vec::from(icmp as &dyn Icmp);
    let checksum = checksum_impl(vec_icmp.as_slice());
    icmp.set_checksum(Some(Self(checksum)));
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
fn test_checksum_impl() {
  assert_eq!(0x210E, checksum_impl(
    &[0x01, 0x00, 0xF2, 0x03, 0xF4, 0xF5, 0xF6, 0xF7]));
  assert_eq!(0x1AFF, checksum_impl(
    &[0xE3, 0x4F, 0x23, 0x96, 0x44, 0x27, 0x99, 0xF3]));
}