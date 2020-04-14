use crate::icmp_type::IcmpType;
use crate::icmp_code::IcmpCode;
use crate::icmp_checksum::IcmpChecksum;

/// [RFC 792](https://tools.ietf.org/html/rfc792)
pub struct Icmp{
  pub type_:IcmpType,
  pub code: IcmpCode,
  pub checksum:IcmpChecksum
}
