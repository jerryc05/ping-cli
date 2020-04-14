use std::ops::Deref;

pub struct IcmpCode(u8);

impl Deref for IcmpCode {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}