use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IcmpCode(pub(crate) u8);

impl Deref for IcmpCode {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl From<u8> for IcmpCode {
  fn from(code: u8) -> Self {
    Self(code)
  }
}