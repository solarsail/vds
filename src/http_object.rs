use std::fmt;

use hyper;
use hyper::header::{Header, HeaderFormat};

use iron::mime::Mime;


#[derive(Debug, Clone)]
pub struct AuthToken(pub String);
impl Header for AuthToken {
    fn header_name() -> &'static str {
        "X-Auth-Token"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<AuthToken> {
        if raw.len() == 1 {
            Ok(AuthToken(String::from_utf8(raw[0].clone()).unwrap()))
        } else {
            Err(hyper::Error::Header)
        }
    }
}

impl HeaderFormat for AuthToken {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone)]
pub struct SubjectToken(pub String);
impl Header for SubjectToken {
    fn header_name() -> &'static str {
        "X-Subject-Token"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<SubjectToken> {
        if raw.len() == 1 {
            Ok(SubjectToken(String::from_utf8(raw[0].clone()).unwrap()))
        } else {
            Err(hyper::Error::Header)
        }
    }
}

impl HeaderFormat for SubjectToken {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn json_mime() -> Mime {
    "application/json".parse().unwrap()
}
