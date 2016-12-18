use std::io::Read;

use iron::prelude::*;
use iron::modifiers;

use hyper::Client;

use iron::status;

use serde_json;
use serde_json::Value;

use json_object;
use http_object;


#[derive(Serialize, Deserialize, Debug)]
struct TokenRequest {
    auth: json_object::Auth,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    token: json_object::Token,
}

pub fn token(req: &mut Request) -> IronResult<Response> {
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

    let mut res = Client::new()
                   .post("http://192.168.1.21:5000/v3/auth/tokens")
                   .body(&serde_json::to_string(&token_req).unwrap())
                   .send().unwrap();
    let mime = http_object::json_mime();
    let mut res_str = String::new();
    res.read_to_string(&mut res_str).unwrap();
    if let Some(token) = res.headers.get::<http_object::SubjectToken>() {
        let body: TokenResponse = serde_json::from_str(&res_str).unwrap();
        println!("{:?}", body);
        Ok(Response::with((status::Ok, mime, modifiers::Header(token.clone()), serde_json::to_string(&body).unwrap())))
    } else {
        let err = json_object::Error { description: "invalid username or password".to_string() };
        Ok(Response::with((status::Unauthorized, mime, serde_json::to_string(&err).unwrap())))
    }
}
