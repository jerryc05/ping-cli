#[allow(unused_imports)]
use crate::icmp::icmp_1_header_0_type_v6::IcmpTypeV6::*;
use std::intrinsics::transmute;

/**
[Reference](https://www.iana.org/assignments/icmpv6-parameters/icmpv6-parameters.xhtml)

|Type     |Name                                                    |Reference
| -       | -                                                      | -
|0        |Reserved
|1        |Destination Unreachable                                 |[RFC4443]
|2        |Packet Too Big  [RFC4443]
|3        |Time Exceeded  [RFC4443]
|4        |Parameter Problem  [RFC4443]
|5-99     |Unassigned
|100      |Private experimentation  [RFC4443]
|101      |Private experimentation  [RFC4443]
|102-126  |Unassigned
|127      |Reserved for expansion of ICMPv6 error messages  [RFC4443]
|128      |Echo Request  [RFC4443]
|129      |Echo Reply  [RFC4443]
|130      |Multicast Listener Query  [RFC2710]
|131      |Multicast Listener Report  [RFC2710]
|132      |Multicast Listener Done  [RFC2710]
|133      |Router Solicitation  [RFC4861]
|134      |Router Advertisement  [RFC4861]
|135      |Neighbor Solicitation  [RFC4861]
|136      |Neighbor Advertisement  [RFC4861]
|137      |Redirect Message  [RFC4861]
|138      |Router Renumbering  [RFC2894]
|139      |ICMP Node Information Query  [RFC4620]
|140      |ICMP Node Information Response  [RFC4620]
|141      |Inverse Neighbor Discovery Solicitation Message  [RFC3122]
|142      |Inverse Neighbor Discovery Advertisement Message  [RFC3122]
|143      |Version 2 Multicast Listener Report  [RFC3810]
|144      |Home Agent Address Discovery Request Message  [RFC6275]
|145      |Home Agent Address Discovery Reply Message  [RFC6275]
|146      |Mobile Prefix Solicitation  [RFC6275]
|147      |Mobile Prefix Advertisement  [RFC6275]
|148      |Certification Path Solicitation Message  [RFC3971]
|149      |Certification Path Advertisement Message  [RFC3971]
|150      |ICMP messages utilized by experimental mobility protocols such as Seamoby  [RFC4065]
|151      |Multicast Router Advertisement  [RFC4286]
|152      |Multicast Router Solicitation  [RFC4286]
|153      |Multicast Router Termination  [RFC4286]
|154      |FMIPv6 Messages  [RFC5568]
|155      |RPL Control Message  [RFC6550]
|156      |ILNPv6 Locator Update Message  [RFC6743]
|157      |Duplicate Address Request  [RFC6775]
|158      |Duplicate Address Confirmation  [RFC6775]
|159      |MPL Control Message  [RFC7731]
|160      |Extended Echo Request  [RFC8335]
|161      |Extended Echo Reply  [RFC8335]
|162-199  |Unassigned
|200      |Private experimentation  [RFC4443]
|201      |Private experimentation  [RFC4443]
|255      |Reserved for expansion of ICMPv6 informational messages |[RFC4443]
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpTypeV6 {
  EchoRequest = 128,
  EchoReply = 129,
  // EchoReply = 0,
  // _Unassigned1To2 = 2,
  // DestinationUnreachable = 3,
  // _SourceQuench = 4,
  // Redirect = 5,
  // _AlternateHostAddress = 6,
  // _Unassigned7 = 7,
  // Echo = 8,
  // RouterAdvertisement = 9,
  // RouterSolicitation = 10,
  // TimeExceeded = 11,
  // ParameterProblem = 12,
  // Timestamp = 13,
  // TimestampReply = 14,
  // _InformationRequest = 15,
  // _InformationReply = 16,
  // _AddressMaskRequest = 17,
  // _AddressMaskReply = 18,
  // _Reserved19To29 = 29,
  // _Traceroute = 30,
  // _DatagramConversionError = 31,
  // _MobileHostRedirect = 32,
  // _IPv6WhereAreYou = 33,
  // _IPv6IAmHere = 34,
  // _MobileRegistrationRequest = 35,
  // _MobileRegistrationReply = 36,
  // _DomainNameRequest = 37,
  // _DomainNameReply = 38,
  // _Skip = 39,
  // Photuris = 40,
  // IcmpMessagesUtilizedByExperimentalMobilityProtocolsSuchAsSeamoby = 41,
  // ExtendedEchoRequest = 42,
  // ExtendedEchoReply = 43,
  // _Unassigned44To252 = 252,
  // Rfc3692StyleExperiment1 = 253,
  // Rfc3692StyleExperiment2 = 254,
  // _Reserved255 = 255,
}

impl From<&IcmpTypeV6> for u8 {
  fn from(type_: &IcmpTypeV6) -> Self {
    (*type_) as Self
  }
}

#[test]
fn test_icmp_type_v6_to_u8() {
//   assert_eq!(0_u8, (&EchoReply).into());
//   assert!(1_u8 <= (&_Unassigned1To2).into() && 2_u8 >= (&_Unassigned1To2).into());
//   assert_eq!(3_u8, (&DestinationUnreachable).into());
//   assert_eq!(16_u8, (&_InformationReply).into());
//   assert!(19_u8 <= (&_Reserved19To29).into() && 29_u8 >= (&_Reserved19To29).into());
  assert_eq!(128_u8, (&EchoRequest).into());
  assert_eq!(129_u8, (&EchoReply).into());
//   assert_eq!(255_u8, (&_Reserved255).into());
}

#[allow(unused_mut, clippy::fallible_impl_from)]
impl From<u8> for IcmpTypeV6 {
  fn from(mut num: u8) -> Self {
    /* special cases */ {
      if num != 128 && num != 129 {
        unimplemented!()
      }
    }

    unsafe { transmute(num) }
  }
}

#[test]
fn test_u8_to_icmp_type_v6() {
//   assert_eq!(EchoReply, IcmpTypeV6::from(0));
//   for i in 1..=2 {
//     assert_eq!(_Unassigned1To2, IcmpTypeV6::from(i));
//   }
//   assert_eq!(DestinationUnreachable, IcmpTypeV6::from(3));
//   assert_eq!(_InformationReply, IcmpTypeV6::from(16));
//   for i in 19..=29 {
//     assert_eq!(_Reserved19To29, IcmpTypeV6::from(i));
//   }
  assert_eq!(EchoRequest, IcmpTypeV6::from(128));
  assert_eq!(EchoReply, IcmpTypeV6::from(129));
//   assert_eq!(_Reserved255, IcmpTypeV6::from(255));
}