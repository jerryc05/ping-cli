use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IcmpCode(u8);

impl Deref for IcmpCode {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}