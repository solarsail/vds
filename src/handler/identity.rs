use std::io::Read;
use std::fmt;

use iron::prelude::*;
use iron::modifiers;

use hyper;
use hyper::Client;
use hyper::header::{Header, HeaderFormat};

use iron::mime::Mime;
use iron::status;

use serde_json;
use serde_json::Value;

use json_object;


#[derive(Debug, Clone)]
struct AuthToken(String);
impl Header for AuthToken {
    fn header_name() -> &'static str {
        "X-Subject-Token"
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

#[derive(Serialize, Deserialize, Debug)]
struct TokenRequest {
    auth: json_object::Auth,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    token: json_object::Token,
}

pub fn login(req: &mut Request) -> IronResult<Response> {
    let mut data = String::new();
    req.body.read_to_string(&mut data).unwrap();
    let data: Value = serde_json::from_str(&data).unwrap();
    let data = data.as_object().unwrap();
    // make json
    let token_req = TokenRequest {
        auth: json_object::Auth {
            identity: json_object::Identity {
                methods: vec!["password".to_string()],
                password: json_object::Password {
                    user: json_object::User {
                        id: String::new(),
                        name: data.get("username").unwrap().as_str().unwrap().to_string(),
                        password: data.get("password").unwrap().as_str().unwrap().to_string(),
                        domain: json_object::Domain {
                            id: "default".to_string(),
                            name: String::new(),
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
    if let Some(token) = res.headers.get::<AuthToken>() {
        let body: TokenResponse = serde_json::from_str(&res_str).unwrap();
        println!("{:?}", body);
        Ok(Response::with((status::Ok, mime, modifiers::Header(token.clone()), serde_json::to_string(&body).unwrap())))
    } else {
        let err = json_object::Error { description: "invalid username or password".to_string() };
        Ok(Response::with((status::Unauthorized, mime, serde_json::to_string(&err).unwrap())))
    }
}
