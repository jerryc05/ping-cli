#![feature(try_trait)]

pub mod icmp;
pub mod utils;
pub mod io;

pub use icmp::known_structs::echo::EchoIcmp;
pub use io::icmp_sender::{ping, PingTimeout};
pub use utils::MyErr;