pub enum IcmpCode {}

impl From<&IcmpCode> for u8 {
  fn from(_: &IcmpCode) -> Self {
    unimplemented!()
  }
}