#[allow(unused_imports)]
use crate::icmp_0_type::IcmpType::*;
use std::intrinsics::transmute;

/**
[Reference](https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml)

|Type    |Name                                     |Reference
| -      | -                                       | -
|0       |Echo Reply                               |[RFC792]
|1       |Unassigned
|2       |Unassigned
|3       |Destination Unreachable                  |[RFC792]
|4       |Source Quench (Deprecated)               |[RFC792][RFC6633]
|5       |Redirect                                 |[RFC792]
|6       |Alternate Host Address (Deprecated)      |[RFC6918]
|7       |Unassigned
|8       |Echo                                     |[RFC792]
|9       |Router Advertisement                     |[RFC1256]
|10      |Router Solicitation                      |[RFC1256]
|11      |Time Exceeded                            |[RFC792]
|12      |Parameter Problem                        |[RFC792]
|13      |Timestamp                                |[RFC792]
|14      |Timestamp Reply                          |[RFC792]
|15      |Information Request (Deprecated)         |[RFC792][RFC6918]
|16      |Information Reply (Deprecated)           |[RFC792][RFC6918]
|17      |Address Mask Request (Deprecated)        |[RFC950][RFC6918]
|18      |Address Mask Reply (Deprecated)          |[RFC950][RFC6918]
|19      |Reserved (for Security)                  |[Solo]
|20-29   |Reserved (for Robustness Experiment)     |[ZSu]
|30      |Traceroute (Deprecated)                  |[RFC1393][RFC6918]
|31      |Datagram Conversion Error (Deprecated)   |[RFC1475][RFC6918]
|32      |Mobile Host Redirect (Deprecated)        |[David_Johnson][RFC6918]
|33      |IPv6 Where-Are-You (Deprecated)          |[Simpson][RFC6918]
|34      |IPv6 I-Am-Here (Deprecated)              |[Simpson][RFC6918]
|35      |Mobile Registration Request (Deprecated) |[Simpson][RFC6918]
|36      |Mobile Registration Reply (Deprecated)   |[Simpson][RFC6918]
|37      |Domain Name Request (Deprecated)         |[RFC1788][RFC6918]
|38      |Domain Name Reply (Deprecated)           |[RFC1788][RFC6918]
|39      |SKIP (Deprecated)                        |[Markson][RFC6918]
|40      |Photuris                                 |[RFC2521]
|41      |ICMP messages utilized by experimental mobility protocols such as Seamoby|[RFC4065]
|42      |Extended Echo Request                    |[RFC8335]
|43      |Extended Echo Reply                      |[RFC8335]
|44-252  |Unassigned
|253     |RFC3692-style Experiment 1               |[RFC4727]
|254     |RFC3692-style Experiment 2               |[RFC4727]
|255     |Reserved                                 |[JBP]
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcmpType {
  EchoReply = 0,
  _Unassigned1To2 = 2,
  DestinationUnreachable = 3,
  _SourceQuench = 4,
  Redirect = 5,
  _AlternateHostAddress = 6,
  _Unassigned7 = 7,
  Echo = 8,
  RouterAdvertisement = 9,
  RouterSolicitation = 10,
  TimeExceeded = 11,
  ParameterProblem = 12,
  Timestamp = 13,
  TimestampReply = 14,
  _InformationRequest = 15,
  _InformationReply = 16,
  _AddressMaskRequest = 17,
  _AddressMaskReply = 18,
  _Reserved19To29 = 29,
  _Traceroute = 30,
  _DatagramConversionError = 31,
  _MobileHostRedirect = 32,
  _IPv6WhereAreYou = 33,
  _IPv6IAmHere = 34,
  _MobileRegistrationRequest = 35,
  _MobileRegistrationReply = 36,
  _DomainNameRequest = 37,
  _DomainNameReply = 38,
  _Skip = 39,
  Photuris = 40,
  IcmpMessagesUtilizedByExperimentalMobilityProtocolsSuchAsSeamoby = 41,
  ExtendedEchoRequest = 42,
  ExtendedEchoReply = 43,
  _Unassigned44To252 = 252,
  Rfc3692StyleExperiment1 = 253,
  Rfc3692StyleExperiment2 = 254,
  _Reserved255 = 255,
}

impl From<&IcmpType> for u8 {
  fn from(type_: &IcmpType) -> Self {
    (*type_) as Self
  }
}

#[test]
fn test_icmp_type_to_u8() {
  assert_eq!(0_u8, (&EchoReply).into());
  assert!(1_u8 <= (&_Unassigned1To2).into() && 2_u8 >= (&_Unassigned1To2).into());
  assert_eq!(3_u8, (&DestinationUnreachable).into());
  assert_eq!(16_u8, (&_InformationReply).into());
  assert!(19_u8 <= (&_Reserved19To29).into() && 29_u8 >= (&_Reserved19To29).into());
  assert_eq!(255_u8, (&_Reserved255).into());
}

impl From<u8> for IcmpType {
  fn from(mut num: u8) -> Self {
    /* special cases */ {
      if 1 <= num && num <= 2 {
        num = 2;
      }
      if 19 <= num && num <= 29 {
        num = 29;
      }
      if 44 <= num && num <= 252 {
        num = 252
      }
    }

    unsafe { transmute(num) }
  }
}

#[test]
fn test_u8_to_icmp_type() {
  assert_eq!(EchoReply, IcmpType::from(0));
  for i in 1..=2 {
    assert_eq!(_Unassigned1To2, IcmpType::from(i));
  }
  assert_eq!(DestinationUnreachable, IcmpType::from(3));
  assert_eq!(_InformationReply, IcmpType::from(16));
  for i in 19..=29 {
    assert_eq!(_Reserved19To29, IcmpType::from(i));
  }
  assert_eq!(_Reserved255, IcmpType::from(255));
}