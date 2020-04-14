use crate::icmp::icmp_1_header_0_type_v4::IcmpTypeV4;
use crate::icmp::icmp_1_header_0_type_v6::IcmpTypeV6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
  V4(IcmpTypeV4),
  V6(IcmpTypeV6),
}

impl From<&IcmpType> for u8 {
  fn from(type_: &IcmpType) -> Self {
    match *type_ {
      IcmpType::V4(type_) => type_ as Self,
      IcmpType::V6(type_) => type_ as Self,
    }
  }
}