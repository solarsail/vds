use std::io::Read;

use iron::prelude::*;
use iron::status;

use router::Router;

use hyper::Client;

use serde_json;

use json_object;
use http_object;


#[derive(Serialize, Deserialize, Debug)]
struct TenantUsageStatResponse {
    tenant_usage: json_object::TenantUsage,
    //tenant_usage_links: Vec<json_object::TenantUsageLink>,
}

pub fn usage(req: &mut Request) -> IronResult<Response> {
    let ref id = req.extensions.get::<Router>().unwrap().find("project_id").unwrap_or("/");
    let mime = http_object::json_mime();
    if let Some(token) = req.headers.get::<http_object::AuthToken>() {
        let client = Client::new();
        let req = client.get(&format!("http://192.168.1.21:8774/v2.1/os-simple-tenant-usage/{}", id));
        let mut res = req.header(token.clone()).send().unwrap();

        let mut res_str = String::new();
        res.read_to_string(&mut res_str).unwrap();

        if let Ok(body) = serde_json::from_str::<TenantUsageStatResponse>(&res_str) {
            println!("{:?}", body);
            Ok(Response::with((status::Ok, mime, serde_json::to_string(&body).unwrap())))
        } else if res.status == status::Forbidden {
            let err = json_object::Error { description: "project not exist or not allowed to access".to_string() };
            Ok(Response::with((res.status, mime, serde_json::to_string(&err).unwrap())))
        } else {
            Ok(Response::with((res.status, res_str)))
        }
    } else {
        let err = json_object::Error { description: "invalid token".to_string() };
        Ok(Response::with((status::Unauthorized, mime, serde_json::to_string(&err).unwrap())))
    }

}
