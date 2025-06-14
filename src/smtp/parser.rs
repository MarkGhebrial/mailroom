//! nom parsers for SMTP
//!
//! See RFC 5321 for the SMTP syntax specifications

use abnf_core::streaming::sp;
use nom::{bytes::streaming::tag, IResult};

// fn crlf(s: &str) -> IResult<&str, &str> {
//     tag("\r\n")(s)
// }

// fn sp(s: &str) -> IResult<&str, &str> {
//     tag(" ")(s)
// }
