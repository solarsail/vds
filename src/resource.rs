use std::io::Read;
use std::ops::Deref;
use std::fmt::Write;
use std::collections::HashMap;

use hyper::Client;
use hyper::method::Method;

use http_object;

#[derive(Debug)]
pub struct Resource {
    url: String,
}

#[derive(Debug)]
pub struct Reply {
    pub code: Status,
    pub token: Option<String>,
    pub content: String,
}

#[derive(Debug)]
pub struct Error {
    code: Status,
    description: String,
}

#[derive(Debug)]
pub enum Status {
    Ok,
    NotFound,
    Unauthorized,
}

impl Resource {
    pub fn new<S: Into<String>>(url: S) -> Resource {
        let mut url = url.into();
        while url.rfind('/') == Some(url.len() - 1) {
            url.pop().unwrap();
        }
        Resource {
            url: url,
        }
    }

    fn request(&self, method: Method, param: Option<&str>) -> Result<Reply, Error> {
        let client = Client::new();
        let builder = client.request(method, &self.url);
        let builder = match param {
            Some(p) => builder.body(p),
            None => builder
        };

        let res = builder.send();
        match res {
            Ok(mut r) => {
                let mut reply = Reply {
                    code: Status::Ok,
                    token: None,
                    content: String::new(),
                };
                if let Some(token) = r.headers.get::<http_object::SubjectToken>() {
                    reply.token = Some(token.0.clone());
                }
                r.read_to_string(&mut reply.content).unwrap();
                Ok(reply)
            }
            Err(e) => {
                Err(Error {
                    code: Status::NotFound,
                    description: String::from("test"),
                })
            }
        }
    }

    pub fn post(&self, param: &str) -> Result<Reply, Error> {
        self.request(Method::Post, Some(param))
    }

    pub fn get(&self, param: Option<&HashMap<&str, &str>>) -> Result<Reply, Error> {
        if let Some(map) = param {
            let mut query = String::new();
            for (key, value) in map {
                write!(&mut query, "{}={}&", key, value).unwrap();
            }
            self.with_query(&query).request(Method::Get, None)
        } else {
            self.request(Method::Get, None)
        }
    }

    pub fn with_item<S: Deref<Target=str>>(&self, id: S) -> Resource {
        let mut r = Resource {
            url: String::with_capacity(self.url.len() + &id.len() + 1),
        };
        write!(&mut r.url, "{}/{}", self.url, id.deref()).unwrap();
        r
    }

    pub fn with_query(&self, query: &str) -> Resource {
        let mut r = Resource {
            url: String::with_capacity(self.url.len() + query.len() + 1),
        };
        write!(&mut r.url, "{}?{}", self.url, query).unwrap();
        r
    }
}
