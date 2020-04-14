use crate::icmp_type::IcmpType;
use crate::icmp_code::IcmpCode;
use crate::icmp_checksum::IcmpChecksum;

pub struct Icmp{
  pub type_:IcmpType,
  pub code: IcmpCode,
  pub checksum:IcmpChecksum
}
