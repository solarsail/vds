use iron::prelude::*;
use iron::modifiers;
use hyper;
use hyper::Client;
use hyper::header::{Header, HeaderFormat};
use std::io::Read;
use std::fmt;
use iron::mime::Mime;
use iron::status;
use serde_json;
use serde_json::Value;

#[derive(Debug, Clone)]
struct Token(String);
impl Header for Token {
    fn header_name() -> &'static str {
        "X-Subject-Token"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<Token> {
        if raw.len() == 1 {
            Ok(Token(String::from_utf8(raw[0].clone()).unwrap()))
        } else {
            Err(hyper::Error::Header)
        }
    }
}

impl HeaderFormat for Token {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Serialize, Deserialize)]
struct Domain {
    id: String,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    domain: Domain,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Password {
    user: User
}

#[derive(Serialize, Deserialize)]
struct Identity {
    methods: Vec<String>,
    password: Password,
}

#[derive(Serialize, Deserialize)]
struct Auth {
    identity: Identity,
}

#[derive(Serialize, Deserialize)]
struct TokenRequest {
    auth: Auth,
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    let mut data = String::new();
    req.body.read_to_string(&mut data).unwrap();
    let data: Value = serde_json::from_str(&data).unwrap();
    let data = data.as_object().unwrap();
    // make json
    let token_req = TokenRequest {
        auth: Auth {
            identity: Identity {
                methods: vec!["password".to_string()],
                password: Password {
                    user: User {
                        name: data.get("username").unwrap().as_str().unwrap().to_string(),
                        password: data.get("password").unwrap().as_str().unwrap().to_string(),
                        domain: Domain {
                            id: "default".to_string(),
                        }
                    }
                }
            }
        }
    };

    let client = Client::new();
    let mut res = client.post("http://192.168.1.21:5000/v3/auth/tokens").body(&serde_json::to_string(&token_req).unwrap()).send().unwrap();
    let mime: Mime = "application/json".parse().unwrap();
    let mut res_str = String::new();
    res.read_to_string(&mut res_str).unwrap();
    let token = res.headers.get::<Token>().unwrap();
    Ok(Response::with((status::Ok, mime, modifiers::Header(token.clone()), res_str)))
}
